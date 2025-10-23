 # Proyek Monitoring Suhu dan Kelembapan DHT22 dengan ESP32-S3 dan ThingsBoard

1. Pendahuluan
Proyek ini bertujuan untuk memantau kondisi suhu dan kelembapan menggunakan sensor **DHT22** yang terhubung ke **ESP32-S3**. Data hasil pembacaan sensor dikirim secara realtime ke platform **ThingsBoard Cloud** melalui protokol **MQTT**. Selain itu, perangkat juga mendukung **pembaruan firmware OTA (Over The Air)** sehingga tidak perlu flashing ulang lewat kabel jika ada pembaruan program.

---

2. Komponen yang Digunakan
No	Komponen	Fungsi
1	ESP32-S3 DevKit	Mikrokontroler utama
2	Sensor DHT22	Sensor suhu dan kelembapan
3	Kabel USB Type-C	Menghubungkan ESP32 ke laptop
4	Ubuntu 22.04	Sistem operasi pengembangan
5	ThingsBoard	Platform IoT untuk monitoring data

3. Persiapan
   
Instalasi Dependensi Dasar

sudo apt update
sudo apt install git curl python3 python3-pip -y

Instalasi Rust dan ESP-IDF

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cargo install espup
espup install

Aktifkan environment:

. $HOME/export-esp.sh

- Membuat Proyek Baru

cargo new dht_esp32 --bin
cd dht_esp32

4. Konfigurasi Cargo.toml

Buka file Cargo.toml, lalu isi seperti di bawah ini:

[package]
name = "dht_esp32"
version = "0.1.0"
edition = "2021"
authors = ["Greista Tezar Rizki Saputra"]
resolver = "2"
rust-version = "1.77"

[dependencies]
esp-idf-svc = "0.51"
embedded-svc = "0.28"
dht-sensor = "0.2"
anyhow = "1.0"
serde_json = "1.0"
log = "0.4"
rand = "0.8"

[build-dependencies]
embuild = "0.33"

[package.metadata.esp-idf]
partition_table = "partition_table.csv"

5. Rangkaian DHT22 ke ESP32-S3
DHT22	ESP32-S3	Keterangan
VCC	3.3V	Tegangan kerja
DATA	GPIO4	Jalur data
GND	GND	Ground

    Disarankan menambahkan resistor 10kΩ antara VCC dan DATA biar pembacaan sensor lebih stabil.

6. Kode Program (src/main.rs)

use anyhow::Result;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::Ets, peripherals::Peripherals, prelude::*},
    mqtt::client::*,
    nvs::EspDefaultNvsPartition,
    wifi::*,
    log::EspLogger,
};
use dht_sensor::*;
use serde_json::json;
use std::{thread, time::Duration};
fn main() -> Result<()> {
    EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // Setup WiFi
    let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?;
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "NAMA_WIFI".try_into()?,
        password: "PASSWORD_WIFI".try_into()?,
        ..Default::default()
    }))?;
    wifi.start()?;
    wifi.connect()?;

    while !wifi.is_connected().unwrap() {
        println!("Menghubungkan ke WiFi...");
        thread::sleep(Duration::from_secs(1));
    }
    println!("WiFi berhasil tersambung!");

    // Setup MQTT ThingsBoard
    let mqtt_url = "mqtt://demo.thingsboard.io:1883";
    let access_token = "ACCESS_TOKEN_MU";
    let mqtt_client = EspMqttClient::new(
        mqtt_url,
        &MqttClientConfiguration {
            username: Some(access_token),
            ..Default::default()
        },
        move |_, _| {},
    )?;

    let pin = peripherals.pins.gpio4;
    let mut delay = Ets;

    // Loop baca sensor dan kirim ke ThingsBoard
    loop {
        match dht22::Reading::read(&mut delay, pin) {
            Ok(reading) => {
                let data = json!({
                    "temperature": reading.temperature,
                    "humidity": reading.relative_humidity
                });
                println!("Data terbaca: {:?}", data);
                mqtt_client.publish(
                    "v1/devices/me/telemetry",
                    QoS::AtLeastOnce,
                    false,
                    data.to_string().as_bytes(),
                )?;
            }
            Err(e) => println!("Gagal membaca sensor: {:?}", e),
        }
        thread::sleep(Duration::from_secs(5));
    }
}

7. Build dan Upload ke ESP32

Tambahkan target ESP32-S3:

rustup target add xtensa-esp32s3-none-elf

Build proyek:

cargo build

Cek port board:

ls /dev/ttyUSB*

Upload ke ESP:

espflash flash target/xtensa-esp32s3-none-elf/debug/dht_esp32 --monitor

8. Setting ThingsBoard

    Buka https://demo.thingsboard.io

    Login → pilih Devices → Add New Device

    Buka tab Credentials, salin Access Token

    Tempel token itu di bagian kode (access_token)

    Jalankan board → buka tab Latest Telemetry → data akan muncul otomatis

Contoh data yang diterima:

{
  "temperature": 28.5,
  "humidity": 72.1
}

9. Hasil Uji Coba

Setelah board dijalankan, data suhu dan kelembapan dari DHT22 muncul di ThingsBoard setiap 5 detik.
Data bisa ditampilkan dalam bentuk grafik atau gauge di dashboard ThingsBoard.


Penulis/NRP: Greista Tezar Rizki Saputra/2042231079
           : Zudan Rizky Aditya/2042231007
Jurusan: Teknik Instrumentasi – ITS
Tahun Angkatan: 2023
