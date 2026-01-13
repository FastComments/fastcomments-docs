## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_comment_params | Vec<models::CreateCommentParams> | Ja |  |
| is_live | bool | Nee |  |
| do_spam_check | bool | Nee |  |
| send_emails | bool | Nee |  |
| populate_notifications | bool | Nee |  |

## Antwoord

Retourneert: `Vec<models::SaveComment200Response>`

---