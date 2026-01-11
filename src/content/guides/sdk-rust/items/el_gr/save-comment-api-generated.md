## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| create_comment_params | models::CreateCommentParams | Ναι |  |
| is_live | bool | Όχι |  |
| do_spam_check | bool | Όχι |  |
| send_emails | bool | Όχι |  |
| populate_notifications | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---