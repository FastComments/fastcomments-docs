---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| create_comment_params | Vec<models::CreateCommentParams> | Oui |  |
| is_live | bool | Non |  |
| do_spam_check | bool | Non |  |
| send_emails | bool | Non |  |
| populate_notifications | bool | Non |  |

## Réponse

Retourne : `Vec<models::SaveComment200Response>`

---