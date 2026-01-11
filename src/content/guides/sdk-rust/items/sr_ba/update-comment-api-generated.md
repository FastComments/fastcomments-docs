---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Da |  |
| context_user_id | String | Ne |  |
| do_spam_check | bool | Ne |  |
| is_live | bool | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---