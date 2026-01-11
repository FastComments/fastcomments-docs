---
Завантажити та змінити розмір зображення

## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| file | std::path::PathBuf | Так |  |
| size_preset | models::SizePreset | Ні |  |
| url_id | String | Ні |  |

## Відповідь

Повертає: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---