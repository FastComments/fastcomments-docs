## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| sso | string = "" | 否 |  |

## 回應

回傳: [`Option[GetUserNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count_response.nim)

## 範例

[inline-code-attrs-start title = 'getUserNotificationCount 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getUserNotificationCount(tenantId = "my-tenant-123", sso = "")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]