Загрузить и изменить размер изображения

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| file | std::path::PathBuf | Да |  |
| size_preset | models::SizePreset | Нет |  |
| url_id | String | Нет |  |

## Ответ

Возвращает: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)