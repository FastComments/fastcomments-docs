## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| create_comment_params | models::CreateCommentParams | Sim |  |
| is_live | bool | Não |  |
| do_spam_check | bool | Não |  |
| send_emails | bool | Não |  |
| populate_notifications | bool | Não |  |

## Resposta

Retorna: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---