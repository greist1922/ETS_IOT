 # Proyek Monitoring Suhu dan Kelembapan DHT22 dengan ESP32-S3 dan ThingsBoard

1. Pendahuluan
Proyek ini bertujuan untuk memantau kondisi suhu dan kelembapan menggunakan sensor **DHT22** yang terhubung ke **ESP32-S3**. Data hasil pembacaan sensor dikirim secara realtime ke platform **ThingsBoard Cloud** melalui protokol **MQTT**. Selain itu, perangkat juga mendukung **pembaruan firmware OTA (Over The Air)** sehingga tidak perlu flashing ulang lewat kabel jika ada pembaruan program.

---

2. Alat dan Bahan

    ESP32-S3 Dev Board

    Sensor DHT22

    Kabel jumper dan breadboard

    Koneksi Wi-Fi

    Akun ThingsBoard Cloud

    Laptop dengan sistem operasi Ubuntu

3. Persiapan Awal

    Pastikan Ubuntu sudah terinstal Rust dan toolchain lengkap dengan perintah:

sudo apt update
sudo apt install build-essential pkg-config libssl-dev cmake
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

Instal esp-idf environment:

cargo install espup
espup install
source ~/export-esp.sh

Tambahkan support target ESP32-S3:

rustup target add xtensa-esp32s3-none-elf

Pastikan kabel data terhubung dengan benar, lalu cek port:

    ls /dev/ttyUSB*

Struktur Folder Proyek

dev/
├── src/
│   └── main.rs
├── Cargo.toml
├── partition_table.csv
├── build.rs
└── README.md

4. Langkah-Langkah Pembuatan
- Buat Proyek Baru

cargo new dev
cd dev

- Tambahkan Dependency

Masukkan semua library yang dibutuhkan pada file Cargo.toml seperti:

    esp-idf-svc

    serde_json

    anyhow

    dht-sensor

    heapless

- Hubungkan Sensor

    VCC DHT22 ke 3.3V ESP32-S3

    GND DHT22 ke GND ESP32-S3

    Data DHT22 ke GPIO4

- Atur Wi-Fi dan Token ThingsBoard

Gunakan SSID dan password Wi-Fi pada konfigurasi, lalu masukkan token akses dari ThingsBoard di bagian MQTT.
- Kompilasi Proyek

Jalankan:

cargo build

- Flash ke ESP32-S3

Gunakan perintah:

espflash flash target/xtensa-esp32s3-none-elf/debug/dev

- Jalankan Serial Monitor

Setelah proses flash selesai, gunakan perintah berikut untuk memantau log:

espflash monitor

Jika semua konfigurasi benar, terminal akan menampilkan log bahwa ESP32 berhasil terkoneksi ke Wi-Fi dan mengirim data ke ThingsBoard Cloud.
Tampilan di ThingsBoard Cloud

    Buka dashboard di ThingsBoard Cloud.

    Buat device baru dengan nama bebas (misalnya: ESP32-DHT22).

    Ambil Access Token dari device tersebut.

    Data suhu dan kelembapan akan muncul secara berkala sesuai interval waktu pengiriman dari ESP32-S3.

5. Troubleshooting

    Jika Wi-Fi tidak tersambung, pastikan SSID dan password benar.

    Jika data tidak muncul di ThingsBoard, periksa token MQTT.

    Pastikan sensor DHT22 tersambung dengan benar (cek wiring dan kabel jumper).

    Gunakan cargo clean bila terjadi error build, lalu build ulang.

6. Hasil dan Analisis

Proyek berhasil mengirimkan data suhu dan kelembapan secara berkala ke platform ThingsBoard. Dengan memanfaatkan Rust dan ESP-IDF, sistem berjalan stabil dan efisien. Nilai pembacaan sensor dapat divisualisasikan dalam bentuk grafik dan tabel di dashboard ThingsBoard.
Kesimpulan

Proyek ini menunjukkan bahwa ESP32-S3 dapat digunakan untuk sistem monitoring IoT berbasis cloud menggunakan ThingsBoard dengan bahasa pemrograman Rust secara real-time.


Penulis/NRP: Greista Tezar Rizki Saputra/2042231079
           : Zudan Rizky Aditya/2042231007
Jurusan: Teknik Instrumentasi – ITS
Tahun Angkatan: 2023
