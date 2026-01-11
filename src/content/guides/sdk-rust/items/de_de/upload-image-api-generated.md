---
Bild hochladen und skalieren

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| file | std::path::PathBuf | Ja |  |
| size_preset | models::SizePreset | Nein |  |
| url_id | String | Nein |  |

## Antwort

Gibt zurück: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---