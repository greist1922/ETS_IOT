Proyek Monitoring Suhu dan Kelembapan DHT22 dengan ESP32-S3 dan ThingsBoard
1. Pendahuluan

Proyek ini dibuat dengan tujuan untuk memantau kondisi suhu dan kelembapan lingkungan secara real-time menggunakan sensor DHT22 yang terhubung ke ESP32-S3. Data dari sensor dikirim ke platform ThingsBoard Cloud melalui protokol MQTT.
Selain itu, sistem ini juga dapat dikembangkan untuk mendukung fitur OTA (Over The Air Update) sehingga pembaruan firmware dapat dilakukan secara jarak jauh tanpa harus menggunakan kabel USB untuk flashing ulang.

Proyek ini menjadi contoh penerapan Internet of Things (IoT) di bidang monitoring lingkungan, di mana sensor dan mikrokontroler bekerja sama untuk mengumpulkan serta mengirimkan data ke platform cloud secara otomatis.
2. Alat dan Bahan

Berikut alat dan bahan yang digunakan dalam proyek ini:

    ESP32-S3 Dev Board

    Sensor DHT22

    Kabel jumper dan breadboard

    Koneksi Wi-Fi

    Akun ThingsBoard Cloud

    Laptop dengan sistem operasi Ubuntu

3. Persiapan Awal

Sebelum memulai pembuatan sistem, lakukan beberapa langkah instalasi dan konfigurasi berikut:
a. Instalasi Rust dan Toolchain

Pastikan Ubuntu sudah terinstal Rust dan toolchain yang diperlukan.

sudo apt update
sudo apt install build-essential pkg-config libssl-dev cmake
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

b. Instalasi ESP-IDF Environment

ESP-IDF digunakan untuk mendukung kompilasi dan komunikasi dengan perangkat ESP32.

cargo install espup
espup install
source ~/export-esp.sh

c. Tambahkan Target untuk ESP32-S3

rustup target add xtensa-esp32s3-none-elf

d. Cek Port ESP32-S3

Pastikan board sudah terhubung dengan benar.

ls /dev/ttyUSB*

4. Struktur Folder Proyek

Struktur proyek secara umum adalah sebagai berikut:

dev/
├── src/
│   └── main.rs
├── Cargo.toml
├── partition_table.csv
├── build.rs
└── README.md

5. Langkah-Langkah Pembuatan
a. Membuat Proyek Baru

cargo new dev
cd dev

b. Menambahkan Dependency

Tambahkan library berikut di dalam file Cargo.toml:

    esp-idf-svc

    serde_json

    anyhow

    dht-sensor

    heapless

Library tersebut digunakan untuk menangani komunikasi MQTT, pengolahan JSON, pembacaan sensor, serta logging sistem.
c. Menghubungkan Sensor DHT22 ke ESP32-S3

Hubungkan pin sensor dengan mikrokontroler sebagai berikut:
Pin DHT22	Pin ESP32-S3	Keterangan
VCC	3.3V	Daya utama
GND	GND	Ground
DATA	GPIO4	Jalur data sensor
d. Mengatur Koneksi Wi-Fi dan ThingsBoard

Konfigurasikan SSID dan password Wi-Fi langsung di dalam kode.
Kemudian, masukkan Access Token dari ThingsBoard untuk otentikasi MQTT agar data dapat dikirim ke server dengan benar.
e. Kompilasi Program

Setelah semua konfigurasi selesai, lakukan proses kompilasi:

cargo build

f. Flash ke ESP32-S3

Gunakan perintah berikut untuk mengunggah firmware ke board:

espflash flash target/xtensa-esp32s3-none-elf/debug/dev

g. Monitoring Serial

Untuk melihat log atau data yang dikirim, gunakan:

espflash monitor

Apabila konfigurasi sudah benar, akan muncul log bahwa ESP32-S3 berhasil terkoneksi ke jaringan Wi-Fi dan mulai mengirim data suhu serta kelembapan ke ThingsBoard Cloud.
6. Tampilan di ThingsBoard Cloud

    Masuk ke dashboard ThingsBoard Cloud.

    Buat device baru (contoh: ESP32-DHT22).

    Salin Access Token dari device tersebut ke program.

    Jalankan perangkat dan buka tab “Latest Telemetry”.

    Data suhu dan kelembapan akan muncul secara periodik.

Kamu juga dapat menambahkan widget grafik dan gauge pada dashboard agar data sensor dapat divisualisasikan secara menarik dan informatif.
7. Troubleshooting

Beberapa masalah umum yang mungkin terjadi beserta solusinya:
Permasalahan	Penyebab	Solusi
Wi-Fi tidak tersambung	SSID atau password salah	Periksa kembali konfigurasi Wi-Fi
Data tidak muncul di ThingsBoard	Token MQTT salah	Pastikan Access Token sesuai dengan device di ThingsBoard
Sensor tidak terbaca	Salah pin atau kabel longgar	Cek kembali jalur koneksi dan pastikan pin GPIO benar
Build error	Cache Rust bermasalah	Jalankan cargo clean, lalu build ulang
8. Hasil dan Analisis

Dari hasil pengujian, sistem mampu membaca dan mengirimkan data suhu serta kelembapan dengan interval waktu tertentu secara stabil.
Data yang diterima di ThingsBoard menunjukkan bahwa sensor DHT22 dapat bekerja dengan baik dan akurat untuk kebutuhan monitoring lingkungan.

Implementasi berbasis Rust pada ESP32-S3 memberikan performa yang efisien, ringan, dan stabil. Selain itu, penggunaan ThingsBoard Cloud memungkinkan visualisasi data dalam bentuk grafik dan indikator yang mudah dibaca.
9. Kesimpulan

Proyek ini membuktikan bahwa ESP32-S3 dapat digunakan sebagai perangkat IoT yang handal untuk sistem monitoring berbasis cloud.
Dengan bantuan sensor DHT22, data suhu dan kelembapan dapat dikirimkan secara real-time ke ThingsBoard Cloud melalui MQTT.
Selain efisien, proyek ini juga mudah dikembangkan lebih lanjut, misalnya dengan menambahkan fitur OTA, penyimpanan lokal, atau integrasi dengan sistem automasi lainnya.
10. Identitas Penulis

Nama:

    Greista Tezar Rizki Saputra (NRP: 2042231079)

    Zudan Rizky Aditya (NRP: 2042231007)

Jurusan: Teknik Instrumentasi – Institut Teknologi Sepuluh Nopember (ITS)
Tahun Angkatan: 2023
