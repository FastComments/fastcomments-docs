## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |

## Risposta

Restituisce: [`Option[GetPagesAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pages_api_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getPages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pagesOpt, httpResp) = client.getPages(tenantId = "my-tenant-123")
if pagesOpt.isSome:
  let pages = pagesOpt.get()
  echo pages
else:
  echo "No pages returned"
echo httpResp
[inline-code-end]

---