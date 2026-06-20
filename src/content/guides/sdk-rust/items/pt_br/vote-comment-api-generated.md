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

Retorna: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de vote_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: VoteCommentParams = VoteCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "cmt_8392a1".to_string(),
    url_id: "news/article-2026-06-19-rust-release".to_string(),
    broadcast_id: "broadcast_2026_06".to_string(),
    vote_body_params: models::VoteBodyParams { value: 1 },
    session_id: Some("sess_4f9b2c".to_string()),
    sso: Some("sso_token_abcd1234".to_string()),
};
let response: VoteResponse = vote_comment(&configuration, params).await?;
[inline-code-end]