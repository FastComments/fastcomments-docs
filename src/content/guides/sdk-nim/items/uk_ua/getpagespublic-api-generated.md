Отримує список сторінок для орендаря. Використовується настільним клієнтом FChat для заповнення списку кімнат.
Потребує, щоб `enableFChat` було true у вирішеній кастомній конфігурації кожної сторінки.
Сторінки, що вимагають SSO, фільтруються відповідно до групового доступу користувача, який робить запит.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| options | GetPagesPublicOptions | Ні |  |

## Відповідь

Повертає: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]