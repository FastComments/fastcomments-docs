## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| id | string | Hayır |  |

## Yanıt

Döndürür: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Örnek

[inline-code-attrs-start title = 'deleteV2PageReact Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteV2PageReact(tenantId = "my-tenant-123", urlId = "news/2026/politics-election", id = "react-456")
if response.isSome:
  let react = response.get()
  echo react
else:
  echo "No reaction returned, status: ", httpResponse.status
[inline-code-end]

---