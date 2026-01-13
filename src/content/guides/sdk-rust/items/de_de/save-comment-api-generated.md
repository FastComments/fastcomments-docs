## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_comment_params | models::CreateCommentParams | Ja |  |
| is_live | bool | Nein |  |
| do_spam_check | bool | Nein |  |
| send_emails | bool | Nein |  |
| populate_notifications | bool | Nein |  |

## Antwort

Gibt zurück: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---