## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_api_user_subscription_data | models::UpdateApiUserSubscriptionData | Yes |  |
| user_id | String | No |  |

## Response

Returns: [`UpdateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_subscription_api_response.rs)

## Example

[inline-code-attrs-start title = 'update_subscription Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update_subscription() -> Result<(), Error> {
    let params: UpdateSubscriptionParams = UpdateSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "sub_8f9a2b".to_string(),
        update_api_user_subscription_data: models::UpdateApiUserSubscriptionData {
            plan: "newsletter-weekly".to_string(),
            active: true,
            renewal_period_days: Some(30),
        },
        user_id: Some("user_42".to_string()),
    };
    let response: UpdateSubscriptionApiResponse = update_subscription(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
