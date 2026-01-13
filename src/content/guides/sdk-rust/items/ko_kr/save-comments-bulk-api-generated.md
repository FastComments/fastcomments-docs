---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| create_comment_params | Vec<models::CreateCommentParams> | 예 |  |
| is_live | bool | 아니요 |  |
| do_spam_check | bool | 아니요 |  |
| send_emails | bool | 아니요 |  |
| populate_notifications | bool | 아니요 |  |

## 응답

반환: `Vec<models::SaveComment200Response>`

---