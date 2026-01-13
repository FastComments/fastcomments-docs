---
## 參數

| 名稱 | 類型 | 必需 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| create_comment_params | Vec<models::CreateCommentParams> | 是 |  |
| is_live | bool | 否 |  |
| do_spam_check | bool | 否 |  |
| send_emails | bool | 否 |  |
| populate_notifications | bool | 否 |  |

## 回傳

回傳: `Vec<models::SaveComment200Response>`

---