---
## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Ja |  |
| context_user_id | String | Nein |  |
| do_spam_check | bool | Nein |  |
| is_live | bool | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---