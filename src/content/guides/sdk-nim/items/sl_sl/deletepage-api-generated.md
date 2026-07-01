## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vrne: [`Option[DeletePageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_page_api_response.nim)

## Primer

[inline-code-attrs-start title = 'deletePage Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (deleteRespOpt, httpResp) = client.deletePage(tenantId = "my-tenant-123", id = "news/article-title")
if deleteRespOpt.isSome:
  let deleteResp = deleteRespOpt.get()
  discard deleteResp
discard httpResp
[inline-code-end]