Качване и оразмеряване на изображение

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| file | std::path::PathBuf | Да |  |
| size_preset | models::SizePreset | Не |  |
| url_id | String | Не |  |

## Отговор

Връща: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---