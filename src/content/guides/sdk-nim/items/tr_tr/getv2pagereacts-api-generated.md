## Parametreler

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |

## Yanıt

Döndürür: [`Option[GetV2PageReacts]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_reacts.nim)

## Örnek

[inline-code-attrs-start title = 'getV2PageReacts Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV2PageReacts(tenantId = "my-tenant-123", urlId = "news/elections/2026/us-primary-results")
if response.isSome:
  let reacts = response.get()
  echo "Received reacts: ", $reacts
else:
  echo "No reacts available"
[inline-code-end]

---