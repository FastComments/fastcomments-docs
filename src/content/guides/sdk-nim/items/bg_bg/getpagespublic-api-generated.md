---
Изброява страниците за tenant. Използва се от настолния клиент FChat за да попълни своя списък с канали.
Изисква `enableFChat` да бъде true в разрешената персонализирана конфигурация за всяка страница.
Страниците, които изискват SSO, се филтрират въз основа на груповия достъп на потребителя, който прави заявката.

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Не |  |
| limit | int | Не |  |
| q | string | Не |  |
| sortBy | PagesSortBy | Не |  |
| hasComments | bool | Не |  |

## Отговор

Връща: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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