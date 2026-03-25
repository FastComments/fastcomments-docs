## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| from_name | String | Sim |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de send_invite'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_invite_example() -> Result<FlagCommentPublic200Response, Error> {
    let params: SendInviteParams = SendInviteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-03-25-modern-rust".to_string(),
        from_name: "Acme Newsroom".to_string(),
        message: Some("Please join the discussion on this article.".to_string()),
    };
    let response: FlagCommentPublic200Response = send_invite(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---