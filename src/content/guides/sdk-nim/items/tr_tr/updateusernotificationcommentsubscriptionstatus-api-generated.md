Belirli bir yorum için bildirimleri etkinleştirir veya devre dışı bırakır.

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| notificationId | string | Hayır |  |
| optedInOrOut | string | Hayır |  |
| commentId | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationCommentSubscriptionStatus(
  tenantId = "my-tenant-123",
  notificationId = "",
  optedInOrOut = "",
  commentId = "cmt-789",
  sso = ""
)

if response.isSome:
  let updateResp = response.get()
  echo "Subscription update response: ", updateResp
[inline-code-end]

---