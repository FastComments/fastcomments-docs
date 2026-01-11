---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| comment_id | String | Evet |  |
| broadcast_id | String | Evet |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Evet |  |
| edit_key | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

---