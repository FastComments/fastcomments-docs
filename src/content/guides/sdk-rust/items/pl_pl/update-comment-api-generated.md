## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Tak |  |
| context_user_id | String | Nie |  |
| do_spam_check | bool | Nie |  |
| is_live | bool | Nie |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)