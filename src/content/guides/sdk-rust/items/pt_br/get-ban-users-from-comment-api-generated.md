---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| comment_id | String | Sim |  |
| sso | String | Não |  |

## Resposta

Retorna: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_from_comment_response.rs)

## Exemplo

[inline-code-attrs-start title = 'get_ban_users_from_comment Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetBanUsersFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345".to_string(),
        sso: Some("sso-unique-id".to_string()),
    };
    let _response = get_ban_users_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---