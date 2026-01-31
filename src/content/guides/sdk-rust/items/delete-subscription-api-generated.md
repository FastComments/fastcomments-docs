## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| user_id | String | No |  |

## Response

Returns: [`DeleteSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_subscription_api_response.rs)

## Example

[inline-code-attrs-start title = 'delete_subscription Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let params: DeleteSubscriptionParams = DeleteSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "newsletter-sub-0001".to_string(),
        user_id: Some("user-84b2".to_string()),
    };
    let response: DeleteSubscriptionApiResponse = delete_subscription(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
