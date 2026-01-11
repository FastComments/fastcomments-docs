## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_api_user_subscription_data | models::CreateApiUserSubscriptionData | Yes |  |

## Response

Returns: [`CreateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_subscription_api_response.rs)

## Example

[inline-code-attrs-start title = 'create_subscription Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateSubscriptionParams = CreateSubscriptionParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_api_user_subscription_data: models::CreateApiUserSubscriptionData {
        user_id: "user_42".to_string(),
        channel: "news/article".to_string(),
        resource_id: "article-9876".to_string(),
        plan: Some("pro-monthly".to_string()),
        start_at: Some("2025-11-01T00:00:00Z".to_string()),
        auto_renew: Some(true),
        notes: None,
    },
};
let response: CreateSubscriptionApiResponse = create_subscription(&configuration, params).await?;
[inline-code-end]
