הפעל או השבת התראות עבור דף. כאשר משתמשים מנויים על דף, נוצרות התראות עבור תגובות שורש חדשות, וגם

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| url | string | לא |  |
| pageTitle | string | לא |  |
| subscribedOrUnsubscribed | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateUserNotificationPageSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationPageSubscriptionStatus(
  tenantId = "my-tenant-123",
  urlId = "news/economy/market-rally-2026-06-19",
  url = "",
  pageTitle = "",
  subscribedOrUnsubscribed = "",
  sso = ""
)

if response.isSome:
  let updateResp = response.get()
  echo "Subscription update received: ", updateResp
else:
  echo "No subscription update returned."
[inline-code-end]

---