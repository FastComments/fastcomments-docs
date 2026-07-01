## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | No |  |

## Odpowiedź

Zwraca: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Przykład

[inline-code-attrs-start title = 'deleteV2PageReact Przykład'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeReact, httpResp) = client.deleteV2PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  id = "react-456",
)

if maybeReact.isSome:
  let react = maybeReact.get()
  echo react
[inline-code-end]