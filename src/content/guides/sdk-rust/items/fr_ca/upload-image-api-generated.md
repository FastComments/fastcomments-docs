---
Téléverser et redimensionner une image

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| file | std::path::PathBuf | Oui |  |
| size_preset | models::SizePreset | Non |  |
| url_id | String | Non |  |

## Réponse

Renvoie: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---