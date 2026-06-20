---
## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | float64 | 否 |  |

## 回應

回傳: [`Option[GetModeratorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators_response.nim)

## 範例

[inline-code-attrs-start title = 'getModerators 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let moderators = response.get()
  echo "Received moderators response:", moderators
else:
  echo "No moderators returned"
[inline-code-end]

---