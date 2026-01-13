## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Ja |  |
| context_user_id | String | Nej |  |
| do_spam_check | bool | Nej |  |
| is_live | bool | Nej |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)