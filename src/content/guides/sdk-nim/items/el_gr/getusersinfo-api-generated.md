---
Συγκεντρωμένες πληροφορίες χρηστών για έναν tenant. Δεδομένων των userIds, επιστρέφει πληροφορίες εμφάνισης από User / SSOUser.
Χρησιμοποιείται από το widget σχολίων για τον εμπλουτισμό χρηστών που μόλις εμφανίστηκαν μέσω ενός γεγονότος παρουσίας.
Χωρίς πλαίσιο σελίδας: η ιδιωτικότητα εφαρμόζεται ομοιόμορφα (τα ιδιωτικά προφίλ αποκρύπτονται).

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## Απόκριση

Επιστρέφει: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---