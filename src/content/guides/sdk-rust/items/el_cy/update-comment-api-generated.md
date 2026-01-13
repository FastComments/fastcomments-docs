## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Ναι |  |
| context_user_id | String | Όχι |  |
| do_spam_check | bool | Όχι |  |
| is_live | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---