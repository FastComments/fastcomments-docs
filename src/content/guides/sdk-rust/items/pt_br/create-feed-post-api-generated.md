## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| create_feed_post_params | models::CreateFeedPostParams | Sim |  |
| broadcast_id | String | Não |  |
| is_live | bool | Não |  |
| do_spam_check | bool | Não |  |
| skip_dup_check | bool | Não |  |

## Resposta

Retorna: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---