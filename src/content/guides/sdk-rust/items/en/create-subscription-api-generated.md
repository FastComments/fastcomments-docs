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
async fn run() -> Result<CreateSubscriptionApiResponse, Error> {
    let params: CreateSubscriptionParams = CreateSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_user_subscription_data: models::CreateApiUserSubscriptionData {
            user_id: "user-42".to_string(),
            topic: "news/article".to_string(),
            plan: Some("pro".to_string()),
            notify_email: Some("alice@acme-corp.com".to_string()),
            frequency: Some("daily".to_string()),
            metadata: Some({
                let mut m = std::collections::HashMap::new();
                m.insert("source".to_string(), "website".to_string());
                m
            }),
            active: Some(true),
        },
    };

    let response: CreateSubscriptionApiResponse = create_subscription(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
