---
Otpremi i promijeni veličinu slike

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| file | std::path::PathBuf | Da |  |
| size_preset | models::SizePreset | Ne |  |
| url_id | String | Ne |  |

## Odgovor

Vraća: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---