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
async fn remove_subscription() -> Result<DeleteSubscriptionApiResponse, Error> {
    let params: DeleteSubscriptionParams = DeleteSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/12345".to_string(),
        user_id: Some("user-9876".to_string()),
    };
    let response: DeleteSubscriptionApiResponse = delete_subscription(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
