## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Risposta

Restituisce: [`Option[GetV2PageReacts]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_reacts.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getV2PageReacts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (reactsOpt, httpResp) = client.getV2PageReacts(tenantId = "my-tenant-123", urlId = "news/article-title")
if reactsOpt.isSome:
  let reacts = reactsOpt.get()
  echo reacts
[inline-code-end]