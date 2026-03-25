## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenant_id | String | Ja |  |
| create_api_user_subscription_data | models::CreateApiUserSubscriptionData | Ja |  |

## Antwoord

Geeft terug: [`CreateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_subscription_api_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'create_subscription Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateSubscriptionParams = CreateSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_user_subscription_data: models::CreateApiUserSubscriptionData {
            api_user_id: "api_user_9843".to_string(),
            resource: "news/article".to_string(),
            plan: Some("pro".to_string()),
            start_at: Some("2026-03-01T12:00:00Z".to_string()),
            metadata: Some(std::collections::HashMap::from([
                ("source".to_string(), "signup_form".to_string()),
                ("utm_campaign".to_string(), "spring_launch".to_string()),
            ])),
            active: Some(true),
        },
    };

    let subscription_response: CreateSubscriptionApiResponse =
        create_subscription(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---