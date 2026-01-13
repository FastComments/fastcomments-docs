---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Evet |  |
| context_user_id | String | Hayır |  |
| do_spam_check | bool | Hayır |  |
| is_live | bool | Hayır |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---