List pages for a tenant. Используется настольным клиентом FChat для заполнения списка комнат.
Требуется, чтобы `enableFChat` был true в итоговой пользовательской конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются с учётом прав доступа групп запрашивающего пользователя.

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Нет |  |
| limit | int | Нет |  |
| q | string | Нет |  |
| sortBy | PagesSortBy | Нет |  |
| hasComments | bool | Нет |  |

## Ответ

Возвращает: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(
  tenantId = "my-tenant-123",
  cursor = "",
  limit = 0,
  q = "",
  sortBy = PagesSortBy(0),
  hasComments = false
)

if response.isSome:
  let pages = response.get()
  echo "Retrieved public pages: ", $pages
else:
  echo "No pages returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---