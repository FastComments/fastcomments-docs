## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| user_id | String | Sì |  |

## Risposta

Restituisce: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## Esempio

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id Esempio'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "user-9876".to_string(),
    };
    let _response = get_user_badge_progress_by_user_id(&config, params).await?;
    Ok(())
}
[inline-code-end]