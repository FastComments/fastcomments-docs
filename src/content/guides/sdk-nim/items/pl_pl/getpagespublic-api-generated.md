Wyświetla listę stron dla najemcy. Używane przez klienta desktopowego FChat do wypełniania jego listy pokoi. Wymaga, aby `enableFChat` było ustawione na true w rozstrzygniętej konfiguracji niestandardowej dla każdej strony. Strony, które wymagają SSO, są filtrowane względem uprawnień grupowych użytkownika wykonującego żądanie.

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| cursor | string | Nie |  |
| limit | int | Nie |  |
| q | string | Nie |  |
| sortBy | PagesSortBy | Nie |  |
| hasComments | bool | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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