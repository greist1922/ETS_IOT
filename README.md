# 🚀 ESP32 Rust OTA & ThingsBoard Integration

Proyek ini merupakan implementasi **ESP32 dengan Rust** menggunakan framework `esp-idf-svc`.  
Sistem ini mendukung:
- 🔌 Koneksi WiFi otomatis  
- 🌐 Integrasi MQTT dengan **ThingsBoard Cloud**  
- ⚡ Over-The-Air (OTA) update melalui RPC  
- 🌡️ Sensor DHT22 untuk pembacaan suhu & kelembapan  
- 💾 Pengiriman data ke ThingsBoard secara real-time  

---

## 🧱 Struktur Proyek

├── src/
│ └── main.rs # Program utama ESP32 (WiFi, MQTT, OTA)
├── Cargo.toml # Konfigurasi dependency dan build
├── partition_table.csv # Tabel partisi ESP32 (OTA, SPIFFS, dll)
├── .gitignore # File pengecualian untuk Git
├── LICENSE # Lisensi proyek (MIT)
└── README.md # Dokumentasi proyek


---

## ⚙️ Fitur Utama

### 1. WiFi Connection
Perangkat otomatis terhubung ke jaringan WiFi dengan SSID dan password yang didefinisikan di kode.

### 2. MQTT & ThingsBoard
Koneksi ke broker MQTT ThingsBoard (`mqtt.thingsboard.cloud`) menggunakan access token device.

### 3. OTA Update
- OTA dilakukan melalui **perintah RPC** dengan payload JSON berisi URL firmware baru.  
- Firmware diunduh via HTTP, diverifikasi, lalu menggantikan partisi OTA aktif.

### 4. Sensor DHT22
Membaca data suhu dan kelembapan, dikirimkan secara periodik ke ThingsBoard dalam format JSON.

---

## 🔧 Build & Flash

### 1. Build release
```bash
cargo build --release

2. Flash ke ESP32

espflash flash --partition-table partition_table.csv target/xtensa-esp32-espidf/release/dev --port /dev/ttyUSB0

3. Monitoring log

espflash monitor /dev/ttyUSB0

🔄 OTA Update via ThingsBoard

    Buka ThingsBoard → Device → RPC

    Kirim payload RPC seperti berikut:

{
  "method": "ota_update",
  "params": {
    "ota_url": "http://your-server.com/firmware.bin"
  }
}

    Perangkat akan mengunduh firmware dan restart otomatis setelah update selesai.

🧠 Dibangun Dengan

    🦀 Rust

🧩 esp-idf-svc

☁️ ThingsBoard Cloud

💡 ESP32 DevKitC
👤 Author

Greista Tezar Rizki Saputra
Teknik Instrumentasi, Institut Teknologi Sepuluh Nopember
📍 Madiun, Indonesia
📜 Lisensi

Proyek ini dilisensikan di bawah MIT License

.

    🧠 Catatan: Pastikan ukuran firmware tidak melebihi kapasitas partisi OTA (0x1E0000) agar proses update berjalan lancar.


---

### 🧾 **LICENSE**

```text
MIT License

Copyright (c) 2025 Greista Tezar Rizki Saputra

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

⚙️ .gitignore

# Rust build
/target/
/debug/
Cargo.lock

# ESP-IDF build artifacts
sdkconfig
sdkconfig.old
build/
*.bin
*.elf
*.map

# Logs
*.log

# Editor/OS files
.DS_Store
Thumbs.db
.vscode/
.idea/
*.swp
