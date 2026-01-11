## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| create_comment_params | Vec<models::CreateCommentParams> | Tak |  |
| is_live | bool | Nie |  |
| do_spam_check | bool | Nie |  |
| send_emails | bool | Nie |  |
| populate_notifications | bool | Nie |  |

## Odpowiedź

Zwraca: `Vec<models::SaveComment200Response>`

---