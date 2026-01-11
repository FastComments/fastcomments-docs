---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-------------|
| tenant_id | String | Sim |  |
| create_comment_params | Vec<models::CreateCommentParams> | Sim |  |
| is_live | bool | Não |  |
| do_spam_check | bool | Não |  |
| send_emails | bool | Não |  |
| populate_notifications | bool | Não |  |

## Resposta

Retorna: `Vec<models::SaveComment200Response>`

---