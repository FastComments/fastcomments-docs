---
## Parametre

| Name | Type | Required | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_comment_params | Vec<models::CreateCommentParams> | Ja |  |
| is_live | bool | Nej |  |
| do_spam_check | bool | Nej |  |
| send_emails | bool | Nej |  |
| populate_notifications | bool | Nej |  |

## Respons

Returnerer: `Vec<models::SaveComment200Response>`

---