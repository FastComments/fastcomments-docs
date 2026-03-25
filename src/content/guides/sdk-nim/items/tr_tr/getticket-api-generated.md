## Parameters

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |
| userId | string | Hayır |  |

## Yanıt

Döndürür: [`Option[GetTicket_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket200response.nim)

## Örnek

[inline-code-attrs-start title = 'getTicket Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTicket(tenantId = "my-tenant-123", id = "", userId = "")
if response.isSome:
  let ticket = response.get()
  discard ticket
[inline-code-end]

---