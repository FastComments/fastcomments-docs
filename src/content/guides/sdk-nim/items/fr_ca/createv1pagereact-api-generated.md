## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| title | string = "" | Non |  |

## Réponse

Renvoie : [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple createV1PageReact'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pageOpt, httpResp) = client.createV1PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/article-456",
  title = "Breaking News: Nim Takes Over"
)

if pageOpt.isSome:
  let page = pageOpt.get()
  discard page
  discard httpResp
[inline-code-end]