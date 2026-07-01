## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | No |  |

## Odpowiedź

Zwraca: [`Option[GetV2PageReactUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_react_users_response.nim)

## Przykład

[inline-code-attrs-start title = 'getV2PageReactUsers Przykład'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getV2PageReactUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  id = "user-456"
)

if maybeResponse.isSome:
  let resp = maybeResponse.get()
  echo resp
[inline-code-end]