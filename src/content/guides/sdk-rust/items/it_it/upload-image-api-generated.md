---
Carica e ridimensiona un'immagine

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| file | std::path::PathBuf | Sì |  |
| size_preset | models::SizePreset | No |  |
| url_id | String | No |  |

## Risposta

Restituisce: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---