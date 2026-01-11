## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| comment_id | String | Sim |  |
| url_id | String | Sim |  |
| broadcast_id | String | Sim |  |
| vote_body_params | models::VoteBodyParams | Sim |  |
| session_id | String | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---