---
Subir y redimensionar una imagen

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| file | std::path::PathBuf | Sí |  |
| size_preset | models::SizePreset | No |  |
| url_id | String | No |  |

## Respuesta

Devuelve: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---