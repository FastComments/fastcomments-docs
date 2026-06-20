Листа страница за тенант. Користи га FChat десктоп клијент за попуњавање своје листе соба.
Захтева да `enableFChat` буде true у резолвованој прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају у складу са приступом група корисника који шаље захтев.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Не |  |
| limit | int | Не |  |
| q | string | Не |  |
| sortBy | PagesSortBy | Не |  |
| hasComments | bool | Не |  |

## Одговор

Враћа: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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