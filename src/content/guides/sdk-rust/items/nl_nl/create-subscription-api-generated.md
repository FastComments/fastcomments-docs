## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_api_user_subscription_data | models::CreateApiUserSubscriptionData | Yes |  |

## Respons

Retourneert: [`CreateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_subscription_api_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'create_subscription voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let subscription_data = models::CreateApiUserSubscriptionData {
        plan_id: "pro-plan".to_string(),
        trial_period_days: Some(14),
        start_date: Some("2024-01-01".to_string()),
        ..Default::default()
    };
    let params = CreateSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_user_subscription_data: subscription_data,
    };
    let _response: CreateSubscriptionApiResponse = create_subscription(configuration, params).await?;
    Ok(())
}
[inline-code-end]