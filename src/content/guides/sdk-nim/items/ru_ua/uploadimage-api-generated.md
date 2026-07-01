Завантажити та змінити розмір зображення

## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| file | string | Ні |  |
| options | UploadImageOptions | Ні |  |

## Відповідь

Повертає: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## Приклад

[inline-code-attrs-start title = 'uploadImage Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (uploadResult, httpResponse) = client.uploadImage(
  tenantId = "my-tenant-123",
  file = "images/avatar.jpg",
  options = UploadImageOptions()
)

if uploadResult.isSome:
  let result = uploadResult.get()
  # використати результат за потреби
[inline-code-end]