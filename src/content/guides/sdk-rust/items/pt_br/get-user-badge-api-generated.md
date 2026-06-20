## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |

## Resposta

Retorna: [`ApiGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_user_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_user_badge() -> Result<ApiGetUserBadgeResponse, Error> {
    let params: GetUserBadgeParams = GetUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-moderator".to_string(),
        include_inactive: Some(false),
    };
    let badge: ApiGetUserBadgeResponse = get_user_badge(&configuration, params).await?;
    Ok(badge)
}
[inline-code-end]

---