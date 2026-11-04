Выводит список страниц для тенанта. Используется настольным клиентом FChat для заполнения списка комнат.
Требует, чтобы `enableFChat` было равно true в разрешённом кастомном конфиге каждой страницы.
Страницы, требующие SSO, фильтруются по групповому доступу запрашивающего пользователя.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | GetPagesPublicOptions | Нет |  |

## Ответ

Возвращает: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]