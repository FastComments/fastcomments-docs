## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| comment_id | String | Sim |  |
| broadcast_id | String | Sim |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Sim |  |
| edit_key | String | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

---