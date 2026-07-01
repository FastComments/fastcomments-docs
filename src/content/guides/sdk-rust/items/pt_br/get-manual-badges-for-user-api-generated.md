## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| badges_user_id | String | Não |  |
| comment_id | String | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_manual_badges_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_manual_badges_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badges(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetManualBadgesForUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        badges_user_id: Some("user-42".to_string()),
        comment_id: Some("comment-987".to_string()),
        sso: Some("sso-abc123".to_string()),
    };
    let _response: GetUserManualBadgesResponse = get_manual_badges_for_user(config, params).await?;
    Ok(())
}
[inline-code-end]