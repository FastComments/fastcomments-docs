## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_api_user_subscription_data | models::CreateApiUserSubscriptionData | Да |  |

## Отговор

Връща: [`CreateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_subscription_api_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за create_subscription'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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