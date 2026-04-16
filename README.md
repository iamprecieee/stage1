# HNG Stage 1: Profile Management API

A name classification and profile management service built with Rust, integrating external APIs (Genderize, Agify, Nationalize) and persisting results in MongoDB.

## Requirements

- Rust
- Cargo
- MongoDB Atlas
- UUID v7 for all IDs.

## Local Setup

1. **Install Dependencies**:
   ```bash
   cargo build
   ```
2. **Configure Environment**:
   Create a `.env` file with:
   ```env
   DATABASE_URL=mongodb+srv://...
   ```
3. **Run Application**:
   ```bash
   cargo run
   ```

## API Endpoints

### 1. Create Profile

`POST /api/profiles`

- **Body**: `{ "name": "ella" }`
- **Success (201)**: Returns full profile data.
- **Idempotency**: Returns 200 with existing data if name already exists.

### 2. Get Single Profile

`GET /api/profiles/{id}`

- **Success (200)**: Returns full profile data.
- **Error (404)**: Profile not found.

### 3. List All Profiles

`GET /api/profiles`

- **Query Params**: `gender`, `country_id`, `age_group` (case-insensitive).
- **Success (200)**: returns count and restricted profile list.

### 4. Delete Profile

`DELETE /api/profiles/{id}`

- **Success (204)**: No Content.
