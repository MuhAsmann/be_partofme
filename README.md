

```markdown
# Rust Actix Web API with Diesel and PostgreSQL

API sederhana menggunakan **Actix Web** sebagai web framework, **Diesel** sebagai ORM, dan **PostgreSQL** sebagai database.

---

## Prasyarat

- **Rust** (minimal versi 1.65+, disarankan instalasi via [rustup](https://rustup.rs/))
- **PostgreSQL** (pastikan sudah terinstall dan service-nya berjalan)
- **Diesel CLI** (untuk manajemen migrasi dan setup database)
- `cargo` (sudah tersedia jika Rust sudah terinstall)

---

## Instalasi Diesel CLI

Jalankan perintah berikut untuk menginstall Diesel CLI dengan fitur PostgreSQL:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

---

## Setup Database PostgreSQL

1. Buat database baru (ganti `rust_api_db` dengan nama yang diinginkan):

```bash
createdb rust_api_db
```

2. Buat file `.env` di root proyek dan isi dengan konfigurasi koneksi database:

```
DATABASE_URL=postgres://username:password@localhost/rust_api_db
```

Ganti `username` dan `password` dengan kredensial PostgreSQL yang valid.

3. (Opsional) Jalankan migrasi Diesel untuk membuat tabel dan struktur database:

```bash
diesel migration run
```

Jika belum ada migrasi, kamu bisa buat migrasi dengan:

```bash
diesel migration generate create_users
```

Kemudian buat SQL migrasi sesuai schema `users`.

---

## Struktur Proyek

```
src/
 ├── config.rs           # Konfigurasi aplikasi (env, logger, dll)
 ├── db.rs               # Setup koneksi database dan pool
 ├── handlers/           # HTTP handler (controller)
 ├── models/             # Definisi struct model data (User, NewUser)
 ├── routes/             # Routing API (menghubungkan route dengan handler)
 ├── schema.rs           # Diesel schema (table, kolom)
 └── main.rs             # Entry point aplikasi
```

---

## Menjalankan Aplikasi

1. Build aplikasi:

```bash
cargo build
```

2. Jalankan aplikasi:

```bash
cargo run
```

3. Server akan berjalan pada `http://127.0.0.1:8080`.

---

## API Endpoints

### GET `/health`

Health check endpoint, untuk cek apakah server berjalan.

```bash
curl http://localhost:8080/health
```

Response:

```json
{"status":"OK"}
```

---

### GET `/users`

Mengambil semua user dari database.

```bash
curl http://localhost:8080/users
```

Response contoh:

```json
[
  {
    "id": 1,
    "name": "John Doe",
    "email": "john@example.com",
    "password_hash": "hashed_password"
  }
]
```

---

### POST `/users`

Membuat user baru.

Request body (JSON):

```json
{
  "name": "John Doe",
  "email": "john@example.com",
  "password_hash": "hashed_password"
}
```

Curl:

```bash
curl -X POST http://localhost:8080/users \
-H "Content-Type: application/json" \
-d '{"name": "John Doe", "email": "john@example.com", "password_hash": "hashed_password"}'
```

Response contoh (user yang baru dibuat):

```json
{
  "id": 2,
  "name": "John Doe",
  "email": "john@example.com",
  "password_hash": "hashed_password"
}
```

---

### GET DETAIL `/users/{id}`

Mengambil detail user berdasarkan ID.

```bash
curl http://localhost:8080/users/1
```
Response contoh:

```json
{
  "id": 1,
  "name": "John Doe",
  "email": "johndoe@yopmail.com"
}
```

---


### DELETE `/users/{id}`

Menghapus user berdasarkan ID.

```bash
curl -X DELETE http://localhost:8080/users/1
```
Response contoh:

```json
{
  "status": "deleted"
}
```  

---

## Penjelasan Cara Kerja

1. **`main.rs`**  
   - Inisialisasi aplikasi, memuat konfigurasi, membuat pool koneksi DB, dan setup HTTP server Actix.
   - Memasang routing endpoint (seperti `/users` dan `/health`).

2. **`db.rs`**  
   - Mengatur koneksi PostgreSQL menggunakan Diesel dengan connection pooling via `r2d2`.

3. **`schema.rs`**  
   - File yang didefinisikan otomatis oleh Diesel CLI yang memetakan tabel dan kolom dari database.

4. **`models/`**  
   - Berisi definisi struct Rust untuk merepresentasikan data tabel (`User`, `NewUser`) lengkap dengan trait `Serialize` agar bisa dikonversi ke JSON.

5. **`handlers/`**  
   - Tempat fungsi-fungsi HTTP handler yang menerima request, melakukan query ke DB (via Diesel), lalu mengembalikan response JSON.
   - Gunakan `web::block` agar operasi blocking DB tidak mengganggu async runtime Actix.

6. **`routes/`**  
   - Mendefinisikan routing endpoint dan menghubungkannya ke handler yang sesuai.

---

## Tips Debugging

- Aktifkan log debug dengan:

```bash
RUST_LOG=debug cargo run
```

- Pastikan environment variable `DATABASE_URL` sudah benar dan database sudah aktif.
- Pastikan `diesel migration run` sudah dijalankan jika memakai migrasi.

---

## Lisensi

MIT License

---