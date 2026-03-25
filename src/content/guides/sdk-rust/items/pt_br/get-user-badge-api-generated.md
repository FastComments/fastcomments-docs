## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |

## Resposta

Retorna: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_user_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_user_badge() -> Result<GetUserBadge200Response, Error> {
    let params = GetUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "verified-journalist-badge-001".to_string(),
        locale: Some("en-US".to_string()),
    };
    let badge: GetUserBadge200Response = get_user_badge(&configuration, params).await?;
    Ok(badge)
}
[inline-code-end]

---