## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Oui |  |
| context_user_id | String | Non |  |
| do_spam_check | bool | Non |  |
| is_live | bool | Non |  |

## Réponse

Renvoie: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---