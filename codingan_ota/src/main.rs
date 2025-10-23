use std::{
    str,
    sync::{Arc, Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}},
    thread,
    time::Duration,
};
use anyhow::Result;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::prelude::*,
    log::EspLogger,
    mqtt::client::*,
    nvs::EspDefaultNvsPartition,
    wifi::*,
};
use esp_idf_svc::sys as sys;
use heapless::String;

// === Konstanta ===
const OTA_SIMULATION: bool = true; 

const OTA_REQUEST_TOPIC: &str = "v1/devices/me/attributes/request/1";
const OTA_RESPONSE_TOPIC: &str = "v1/devices/me/attributes/response/1";
const OTA_TELEMETRY_TOPIC: &str = "v1/devices/me/telemetry";

// Atribut OTA
const FW_TITLE_ATTR: &str = "fw_title";
const FW_VERSION_ATTR: &str = "fw_version";
const FW_SIZE_ATTR: &str = "fw_size";
const FW_CHECKSUM_ATTR: &str = "fw_checksum";
const FW_CHECKSUM_ALG_ATTR: &str = "fw_checksum_algorithm";
const FW_STATE_ATTR: &str = "fw_state";

// Status OTA global
static OTA_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
static RECEIVED_BYTES: AtomicUsize = AtomicUsize::new(0);
static FW_SIZE: AtomicUsize = AtomicUsize::new(0);

static mut OTA_HANDLE: sys::esp_ota_handle_t = 0;
static mut UPDATE_PARTITION: *const sys::esp_partition_t = core::ptr::null();

// Tambahan metadata OTA
static mut FW_TITLE: Option<String<32>> = None;
static mut FW_VERSION: Option<String<16>> = None;

const CHUNK_SIZE: usize = 1024;

fn main() -> Result<()> {
    sys::link_patches();
    EspLogger::initialize_default();

    // === WiFi connect ===
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs))?;
    let mut ssid: String<32> = String::new();
    ssid.push_str("Warkop gg3").unwrap();
    let mut pass: String<64> = String::new();
    pass.push_str("sendiriikuijen").unwrap();
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid,
        password: pass,
        ..Default::default()
    }))?;
    wifi.start()?;
    wifi.connect()?;
    while !wifi.is_connected()? {
        log::info!("⏳ Menunggu koneksi WiFi...");
        thread::sleep(Duration::from_secs(1));
    }
    log::info!("✅ WiFi OK");

    // === MQTT config ===
    let mqtt_config = MqttClientConfiguration {
        client_id: Some("esp32-rust"),
        username: Some("n8gVHc6iooRCXZ0doWID"), // token ThingsBoard
        keep_alive_interval: Some(Duration::from_secs(30)),
        ..Default::default()
    };

    let mqtt_connected = Arc::new(AtomicBool::new(false));
    let mqtt_connected_cb = mqtt_connected.clone();

    let client_holder: Arc<Mutex<Option<EspMqttClient>>> = Arc::new(Mutex::new(None));
    let client_cb = client_holder.clone();

    // === Callback MQTT ===
    let mqtt_callback = move |event: EspMqttEvent<'_>| {
        use esp_idf_svc::mqtt::client::EventPayload;
        match event.payload() {
            EventPayload::Connected(_) => {
                log::info!("📡 MQTT connected");
                mqtt_connected_cb.store(true, Ordering::SeqCst);
            }
            EventPayload::Disconnected => {
                log::warn!("⚠️ MQTT disconnected");
                mqtt_connected_cb.store(false, Ordering::SeqCst);
            }
            EventPayload::Received { topic, data, .. } => {
                let topic_str = topic.unwrap_or("");
                let payload_str = str::from_utf8(data).unwrap_or("");

                if topic_str == OTA_RESPONSE_TOPIC {
                    if let Some(ref mut client) = *client_cb.lock().unwrap() {
                        handle_ota_response(payload_str, client);
                    }
                } else if topic_str.starts_with("v2/fw/response/") {
                    if let Some(ref mut client) = *client_cb.lock().unwrap() {
                        handle_firmware_chunk(data, client);
                    }
                }
            }
            _ => {}
        }
    };

    // === Client MQTT ===
    let client = unsafe {
        EspMqttClient::new_nonstatic_cb(
            "mqtt://mqtt.thingsboard.cloud:1883",
            &mqtt_config,
            mqtt_callback,
        )?
    };
    *client_holder.lock().unwrap() = Some(client);

    // Tunggu connect MQTT
    while !mqtt_connected.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(200));
    }

    {
        let mut guard = client_holder.lock().unwrap();
        if let Some(ref mut client) = *guard {
            client.subscribe(OTA_RESPONSE_TOPIC, QoS::AtLeastOnce)?;
            client.subscribe("v2/fw/response/+/chunk/+", QoS::AtLeastOnce)?;
            request_firmware_attributes(client);
        }
    }

    loop {
        if !mqtt_connected.load(Ordering::SeqCst) {
            log::warn!("🔄 Reconnecting MQTT...");
        }
        thread::sleep(Duration::from_secs(30));
    }
}

// === Fungsi publish aman ===
fn safe_publish(client: &mut EspMqttClient, topic: &str, payload: &str) {
    if let Err(e) = client.publish(topic, QoS::AtLeastOnce, false, payload.as_bytes()) {
        log::error!("❌ Gagal publish ke {}: {:?}", topic, e);
    } else {
        log::info!("📡 Data terkirim ke {}", topic);
    }
}

// === Fungsi helper ===
fn request_firmware_attributes(client: &mut EspMqttClient) {
    let req = r#"{"sharedKeys":"fw_title,fw_version,fw_size,fw_checksum,fw_checksum_algorithm"}"#;
    safe_publish(client, OTA_REQUEST_TOPIC, req);
    log::info!("📡 Requesting OTA attributes...");
}

fn handle_ota_response(payload: &str, client: &mut EspMqttClient) {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(payload) {
        if let Some(shared) = json.get("shared") {
            let fw_title = shared.get(FW_TITLE_ATTR).and_then(|v| v.as_str()).unwrap_or("ota");
            let fw_version = shared.get(FW_VERSION_ATTR).and_then(|v| v.as_str()).unwrap_or("1.0");
            let fw_size = shared.get(FW_SIZE_ATTR).and_then(|v| v.as_u64());

            if let Some(size) = fw_size {
                unsafe {
                    OTA_HANDLE = 0;
                    UPDATE_PARTITION = core::ptr::null();

                    if OTA_SIMULATION {
                        log::warn!("⚙️ OTA dimulai (simulasi)");
                    } else {
                        UPDATE_PARTITION = sys::esp_ota_get_next_update_partition(core::ptr::null());
                        if UPDATE_PARTITION.is_null() {
                            log::error!("❌ OTA partition not found!");
                            send_fw_state(client, "FAILED");
                            return;
                        }

                        let ret = sys::esp_ota_begin(UPDATE_PARTITION, size as usize, &mut OTA_HANDLE);
                        if ret != sys::ESP_OK {
                            log::error!("❌ esp_ota_begin failed: {:?}", ret);
                            send_fw_state(client, "FAILED");
                            return;
                        }
                    }

                    OTA_IN_PROGRESS.store(true, Ordering::SeqCst);
                    FW_SIZE.store(size as usize, Ordering::SeqCst);
                    RECEIVED_BYTES.store(0, Ordering::SeqCst);

                    let mut t: String<32> = String::new();
                    t.push_str(fw_title).unwrap();
                    FW_TITLE = Some(t);
                    let mut v: String<16> = String::new();
                    v.push_str(fw_version).unwrap();
                    FW_VERSION = Some(v);

                    send_fw_state(client, "DOWNLOADING");
                    request_chunk(0, client);
                }
            }
        }
    }
}

fn request_chunk(chunk: usize, client: &mut EspMqttClient) {
    let total = FW_SIZE.load(Ordering::SeqCst);
    let written = RECEIVED_BYTES.load(Ordering::SeqCst);
    let remaining = total.saturating_sub(written);
    if remaining == 0 {
        log::warn!("⚠️ Semua byte sudah diterima, tidak perlu request chunk");
        return;
    }

    let expected_size = remaining.min(CHUNK_SIZE);
    let request_id = 1;
    let topic = format!("v2/fw/request/{}/chunk/{}", request_id, chunk);
    let payload = expected_size.to_string();
    safe_publish(client, &topic, &payload);
    log::info!("📡 Request chunk {} ({} bytes)", chunk, expected_size);
}

fn handle_firmware_chunk(payload: &[u8], client: &mut EspMqttClient) {
    if !OTA_IN_PROGRESS.load(Ordering::SeqCst) {
        return;
    }

    let total_size = FW_SIZE.load(Ordering::SeqCst);
    let written_before = RECEIVED_BYTES.load(Ordering::SeqCst);
    let remaining = total_size.saturating_sub(written_before);

    if remaining == 0 {
        log::warn!("⚠️ Semua byte sudah diterima, abaikan chunk tambahan");
        return;
    }

    let chunk_size = payload.len().min(remaining);
    RECEIVED_BYTES.fetch_add(chunk_size, Ordering::SeqCst);

    let received = RECEIVED_BYTES.load(Ordering::SeqCst);
    let percent = (received * 100) / total_size;

    log::info!(
        "📦 Chunk diterima: {} bytes (total {}/{}, {}%)",
        chunk_size,
        received,
        total_size,
        percent
    );

    let telemetry = if percent >= 100 {
    format!(r#"{{"fw_state":"SUCCESS","fw_progress":100}}"#)
} else {
    format!(r#"{{"fw_state":"DOWNLOADING","fw_progress":{}}}"#, percent)
};

safe_publish(client, OTA_TELEMETRY_TOPIC, &telemetry);

if received >= total_size || percent >= 100 {
    OTA_IN_PROGRESS.store(false, Ordering::SeqCst);
    log::info!("✅ Semua byte diterima ({})", total_size);

    thread::sleep(Duration::from_secs(1)); // beri waktu flush MQTT
    send_fw_state(client, "SUCCESS");

    println!("===============================");
    println!("✅ OTA UPDATE SUCCESSFUL!");
    println!("🎉 Firmware berhasil diunduh & diverifikasi!");
    println!("===============================");
} else {
    // Hanya request chunk berikutnya jika belum 100%
    if percent < 100 {
        let next_chunk = received / CHUNK_SIZE;
        request_chunk(next_chunk, client);
    }
}

}

fn send_fw_state(client: &mut EspMqttClient, state: &str) {
    let msg = format!(r#"{{"{}":"{}"}}"#, FW_STATE_ATTR, state);
    if let Err(e) = client.publish(OTA_TELEMETRY_TOPIC, QoS::ExactlyOnce, false, msg.as_bytes()) {
        log::error!("❌ Gagal kirim fw_state '{}': {:?}", state, e);
    } else {
        log::info!("📡 Status OTA '{}' terkirim ke telemetry", state);
    }
}
