## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| create_comment_params | models::CreateCommentParams | Evet |  |
| is_live | bool | Hayır |  |
| do_spam_check | bool | Hayır |  |
| send_emails | bool | Hayır |  |
| populate_notifications | bool | Hayır |  |

## Yanıt

Döndürür: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---