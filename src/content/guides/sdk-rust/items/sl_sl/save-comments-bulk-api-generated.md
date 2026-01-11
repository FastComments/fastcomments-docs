---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| create_comment_params | Vec<models::CreateCommentParams> | Da |  |
| is_live | bool | Ne |  |
| do_spam_check | bool | Ne |  |
| send_emails | bool | Ne |  |
| populate_notifications | bool | Ne |  |

## Odgovor

Vrne: `Vec<models::SaveComment200Response>`

---