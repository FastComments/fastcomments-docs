---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| page_size | i32 | Όχι |  |
| after_id | String | Όχι |  |
| include_context | bool | Όχι |  |
| after_created_at | i64 | Όχι |  |
| unread_only | bool | Όχι |  |
| dm_only | bool | Όχι |  |
| no_dm | bool | Όχι |  |
| include_translations | bool | Όχι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---