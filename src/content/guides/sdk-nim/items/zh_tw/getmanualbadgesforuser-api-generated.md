## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetManualBadgesForUserOptions | No |  |

## 回應

回傳: [`Option[GetUserManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_manual_badges_response.nim)

## 範例

[inline-code-attrs-start title = 'getManualBadgesForUser 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (userBadgesOpt, httpResp) = client.getManualBadgesForUser(
  tenantId = "my-tenant-123",
  options = GetManualBadgesForUserOptions()
)
if userBadgesOpt.isSome:
  let badges = userBadgesOpt.get()
  echo badges
[inline-code-end]