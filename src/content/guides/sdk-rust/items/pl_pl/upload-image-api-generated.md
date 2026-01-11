---
Prześlij i zmień rozmiar obrazu

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| file | std::path::PathBuf | Tak |  |
| size_preset | models::SizePreset | Nie |  |
| url_id | String | Nie |  |

## Odpowiedź

Zwraca: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---