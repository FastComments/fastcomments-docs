Enable ή disable ειδοποιήσεις για μια σελίδα. Όταν οι χρήστες είναι εγγεγραμμένοι σε μια σελίδα, δημιουργούνται ειδοποιήσεις για νέα κύρια σχόλια, καθώς και

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| url | string | Όχι |  |
| pageTitle | string | Όχι |  |
| subscribedOrUnsubscribed | string | Όχι |  |
| sso | string = "" | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.updateUserNotificationPageSubscriptionStatus(
  tenantId = "my-tenant-123",
  urlId = "news/article-456",
  url = "https://example.com/news/article-456",
  pageTitle = "Breaking News: Something Happened",
  subscribedOrUnsubscribed = "subscribed",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
  # περαιτέρω επεξεργασία με resp
[inline-code-end]