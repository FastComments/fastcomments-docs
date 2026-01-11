## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| create_comment_params | models::CreateCommentParams | Tak |  |
| is_live | bool | Nie |  |
| do_spam_check | bool | Nie |  |
| send_emails | bool | Nie |  |
| populate_notifications | bool | Nie |  |

## Odpowiedź

Zwraca: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---