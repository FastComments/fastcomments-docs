## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |

## Respons

Retourneert: [`Option[DeletePageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_page_api_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'deletePage voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (deleteRespOpt, httpResp) = client.deletePage(tenantId = "my-tenant-123", id = "news/article-title")
if deleteRespOpt.isSome:
  let deleteResp = deleteRespOpt.get()
  discard deleteResp
discard httpResp
[inline-code-end]