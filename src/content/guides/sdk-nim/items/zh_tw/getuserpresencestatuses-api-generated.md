## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlIdWS | string | 否 |  |
| userIds | string | 否 |  |

## 回應

回傳: [`Option[GetUserPresenceStatuses_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_presence_statuses200response.nim)

## 範例

[inline-code-attrs-start title = 'getUserPresenceStatuses 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserPresenceStatuses(
  tenantId = "my-tenant-123",
  urlIdWS = "news/2025/technology/ai-ethics",
  userIds = "user-789,user-456"
)
if response.isSome:
  let presence = response.get()
  echo "Presence received: ", presence
else:
  echo "No presence information returned, HTTP status: ", httpResponse.status.code
[inline-code-end]

---