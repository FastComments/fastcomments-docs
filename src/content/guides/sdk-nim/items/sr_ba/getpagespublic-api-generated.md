Листати странице за tenant. Користи се од стране FChat десктоп клијента да попуни своју листу соба.
Захтијева да `enableFChat` буде true на решеном прилагођеном конфигу за сваку страницу.
Странице које захтијевају SSO се филтрирају према приступу групе корисника који прави захтев.

## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | int | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | bool | No |  |

## Одговор

Враћа: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

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