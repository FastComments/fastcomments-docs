## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| sso | string = "" | No |  |

## Risposta

Restituisce: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Esempio

[inline-code-attrs-start title = 'putCloseThread Esempio'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.putCloseThread(tenantId = "my-tenant-123", urlId = "news/fastcomments-integration", sso = "")
if respOpt.isSome:
  let empty = respOpt.get()
  echo "Thread successfully closed"
[inline-code-end]