## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| update_api_user_subscription_data | models::UpdateApiUserSubscriptionData | Da |  |
| user_id | String | Ne |  |

## Odgovor

Vrne: [`UpdateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_subscription_api_response.rs)

## Primer

[inline-code-attrs-start title = 'update_subscription Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = UpdateSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "sub-12345".to_string(),
        update_api_user_subscription_data: models::UpdateApiUserSubscriptionData {
            plan_id: "premium".to_string(),
            status: "active".to_string(),
        },
        user_id: Some("user-987".to_string()),
    };
    let _resp = update_subscription(config, params).await?;
    Ok(())
}
[inline-code-end]

---