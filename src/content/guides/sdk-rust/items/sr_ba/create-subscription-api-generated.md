## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_api_user_subscription_data | models::CreateApiUserSubscriptionData | Да |  |

## Одговор

Враћа: [`CreateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_subscription_api_response.rs)

## Пример

[inline-code-attrs-start title = 'create_subscription Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateSubscriptionParams = CreateSubscriptionParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_api_user_subscription_data: models::CreateApiUserSubscriptionData {
        user_id: "user-987".to_string(),
        plan_id: "pro-monthly".to_string(),
        source: Some("website".to_string()),
        topics: Some(vec!["news/article".to_string(), "product/updates".to_string()]),
        auto_renew: Some(true),
        metadata: Some(std::collections::HashMap::from([("ref".to_string(), "signup_form".to_string())])),
    },
};
let response: CreateSubscriptionApiResponse = create_subscription(&configuration, params).await?;
[inline-code-end]