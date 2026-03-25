## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |

## Réponse

Renvoie : [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_user_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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