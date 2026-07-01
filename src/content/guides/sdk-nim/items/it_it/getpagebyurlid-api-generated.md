## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Risposta

Restituisce: [`Option[GetPageByURLIdAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_page_by_urlid_api_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getPageByURLId'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pageOpt, httpRes) = client.getPageByURLId(tenantId = "my-tenant-123", urlId = "news/article-title")
if pageOpt.isSome:
  let page = pageOpt.get()
  echo page
[inline-code-end]