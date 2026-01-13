## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_comment_params | Vec<models::CreateCommentParams> | Ja |  |
| is_live | bool | Nein |  |
| do_spam_check | bool | Nein |  |
| send_emails | bool | Nein |  |
| populate_notifications | bool | Nein |  |

## Antwort

Gibt zurück: `Vec<models::SaveComment200Response>`

---