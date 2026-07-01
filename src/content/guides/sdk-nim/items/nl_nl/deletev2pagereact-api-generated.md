## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| id | string | Nee |  |

## Respons

Retourneert: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Voorbeeld

[inline-code-attrs-start title = 'deleteV2PageReact Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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