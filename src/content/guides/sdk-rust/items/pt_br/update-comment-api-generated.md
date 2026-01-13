## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Sim |  |
| context_user_id | String | Não |  |
| do_spam_check | bool | Não |  |
| is_live | bool | Não |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---