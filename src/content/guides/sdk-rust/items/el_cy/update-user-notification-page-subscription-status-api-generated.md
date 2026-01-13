---
Ενεργοποιήστε ή απενεργοποιήστε τις ειδοποιήσεις για μια σελίδα. Όταν οι χρήστες είναι εγγεγραμμένοι σε μια σελίδα, δημιουργούνται ειδοποιήσεις για νέα σχόλια ρίζας, και επίσης

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| url_id | String | Ναι |  |
| url | String | Ναι |  |
| page_title | String | Ναι |  |
| subscribed_or_unsubscribed | String | Ναι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---