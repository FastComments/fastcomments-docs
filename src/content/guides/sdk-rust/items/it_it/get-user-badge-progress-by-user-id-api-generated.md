## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| user_id | String | Sì |  |

## Risposta

Restituisce: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_by_id_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id Esempio'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetUserBadgeProgressById200Response, Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params: GetUserBadgeProgressByUserIdParams = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "journalist-9876".to_string(),
        include_inactive: Some(false),
        locale: Some("en-US".to_string()),
    };
    let response: GetUserBadgeProgressById200Response =
        get_user_badge_progress_by_user_id(cfg, params).await?;
    Ok(response)
}
[inline-code-end]

---