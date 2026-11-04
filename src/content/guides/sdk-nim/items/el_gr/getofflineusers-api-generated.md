Προηγούμενοι σχολιαστές της σελίδας που ΔΕΝ είναι επί του παρόντος συνδεδεμένοι. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε το μετά την εξάντληση του /users/online για να αποδώσετε μια ενότητα "Members".
Σελιδοποίηση με cursor στο commenterName: ο διακομιστής διασχίζει το μερικό ευρετήριο {tenantId, urlId, commenterName}
από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| options | GetOfflineUsersOptions | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]