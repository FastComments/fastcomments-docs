## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## 例

[inline-code-attrs-start title = 'resetUserNotificationCount の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "sso-jwt-9a8b7c6d")
if response.isSome:
  let resetResult = response.get()
  echo resetResult
else:
  echo "Reset failed, status: ", httpResponse.status
[inline-code-end]

---