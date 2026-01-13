---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| create_comment_params | Vec<models::CreateCommentParams> | Evet |  |
| is_live | bool | Hayır |  |
| do_spam_check | bool | Hayır |  |
| send_emails | bool | Hayır |  |
| populate_notifications | bool | Hayır |  |

## Yanıt

Döndürür: `Vec<models::SaveComment200Response>`

---