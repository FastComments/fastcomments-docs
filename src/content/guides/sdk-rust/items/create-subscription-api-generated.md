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
async fn create_subscription_example() -> Result<(), Error> {
    let params: CreateSubscriptionParams = CreateSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_user_subscription_data: models::CreateApiUserSubscriptionData {
            user_id: "user_12345".to_string(),
            topic: "news/article".to_string(),
            callback_url: Some("https://acme.example.com/webhooks/comments".to_string()),
            channels: Some(vec!["email".to_string(), "webhook".to_string()]),
            active: Some(true),
        },
    };
    let subscription: CreateSubscriptionApiResponse = create_subscription(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
