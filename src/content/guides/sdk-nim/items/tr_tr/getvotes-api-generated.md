## Parametreler

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |

## Yanıt

Döndürür: [`Option[GetVotes_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes200response.nim)

## Örnek

[inline-code-attrs-start title = 'getVotes Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotes(tenantId = "my-tenant-123", urlId = "news/2026/major-update")
if response.isSome:
  let votes = response.get()
  discard votes
else:
  discard httpResponse
[inline-code-end]

---