---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| create_comment_params | Vec<models::CreateCommentParams> | כן |  |
| is_live | bool | לא |  |
| do_spam_check | bool | לא |  |
| send_emails | bool | לא |  |
| populate_notifications | bool | לא |  |

---

## תגובה

מחזיר: `Vec<models::SaveComment200Response>`

---