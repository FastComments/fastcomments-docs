## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Ja |  |
| context_user_id | String | Nee |  |
| do_spam_check | bool | Nee |  |
| is_live | bool | Nee |  |

## Respons

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---