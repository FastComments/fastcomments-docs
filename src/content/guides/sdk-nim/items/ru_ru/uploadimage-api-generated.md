Загрузка и изменение размера изображения

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| file | string | No |  |
| options | UploadImageOptions | No |  |

## Ответ

Возвращает: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## Пример

[inline-code-attrs-start title = 'uploadImage Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (uploadResult, httpResponse) = client.uploadImage(
  tenantId = "my-tenant-123",
  file = "images/avatar.jpg",
  options = UploadImageOptions()
)

if uploadResult.isSome:
  let result = uploadResult.get()
  # использовать результат по мере необходимости
[inline-code-end]