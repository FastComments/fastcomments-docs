---
Отпреми и промијени величину слике

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| file | std::path::PathBuf | Да |  |
| size_preset | models::SizePreset | Не |  |
| url_id | String | Не |  |

## Одговор

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---