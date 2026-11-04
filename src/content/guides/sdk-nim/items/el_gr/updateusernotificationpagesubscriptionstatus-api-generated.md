Ενεργοποίηση ή απενεργοποίηση ειδοποιήσεων για μια σελίδα. Όταν οι χρήστες είναι εγγεγραμμένοι σε μια σελίδα, δημιουργούνται ειδοποιήσεις
για νέα σχόλια ρίζας, καθώς και

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| url | string | Όχι |  |
| pageTitle | string | Όχι |  |
| subscribedOrUnsubscribed | string | Όχι |  |
| sso | string = "" | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateUserNotificationPageSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
  # περαιτέρω επεξεργασία με το resp
[inline-code-end]