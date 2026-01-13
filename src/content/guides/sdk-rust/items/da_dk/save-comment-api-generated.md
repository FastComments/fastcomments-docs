---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_comment_params | models::CreateCommentParams | Ja |  |
| is_live | bool | Nej |  |
| do_spam_check | bool | Nej |  |
| send_emails | bool | Nej |  |
| populate_notifications | bool | Nej |  |

## Respons

Returnerer: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---