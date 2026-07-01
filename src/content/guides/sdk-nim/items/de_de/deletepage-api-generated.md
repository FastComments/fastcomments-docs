## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|---------------|--------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |

## Antwort

Rückgabe: [`Option[DeletePageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_page_api_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deletePage Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (deleteRespOpt, httpResp) = client.deletePage(tenantId = "my-tenant-123", id = "news/article-title")
if deleteRespOpt.isSome:
  let deleteResp = deleteRespOpt.get()
  discard deleteResp
discard httpResp
[inline-code-end]