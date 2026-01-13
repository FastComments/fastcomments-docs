## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| create_comment_params | models::CreateCommentParams | Oui |  |
| is_live | bool | Non |  |
| do_spam_check | bool | Non |  |
| send_emails | bool | Non |  |
| populate_notifications | bool | Non |  |

## Réponse

Renvoie : [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---