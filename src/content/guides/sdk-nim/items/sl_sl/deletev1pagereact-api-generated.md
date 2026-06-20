## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |

## Odgovor

Vrne: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Primer

[inline-code-attrs-start title = 'deleteV1PageReact Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteV1PageReact(tenantId = "my-tenant-123", urlId = "news/article-title")
if response.isSome:
  let deletedReact = response.get()
  echo "Deleted react:", deletedReact
else:
  echo "No react returned for tenant: my-tenant-123, url: news/article-title"
[inline-code-end]

---