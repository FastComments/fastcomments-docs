---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 回應

回傳: [`Option[APIGetUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_response.nim)

## 範例

[inline-code-attrs-start title = 'getUserBadge 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadge(tenantId = "my-tenant-123", id = "badge-9876")
if response.isSome:
  let badge = response.get()
  echo "Fetched badge:"
  echo badge
else:
  echo "No badge found"
  echo httpResponse
[inline-code-end]

---