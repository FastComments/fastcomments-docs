## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| sso | string = "" | Ne |  |

## Odgovor

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'putCloseThread Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.putCloseThread(tenantId = "my-tenant-123", urlId = "news/fastcomments-integration", sso = "")
if respOpt.isSome:
  let empty = respOpt.get()
  echo "Thread successfully closed"
[inline-code-end]