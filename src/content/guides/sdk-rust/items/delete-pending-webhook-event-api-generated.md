## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_pending_webhook_event Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_delete_pending_webhook_event() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeletePendingWebhookEventParams = DeletePendingWebhookEventParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "wh_evt_news_20260113_9b7f".to_string(),
        reason: Some("cleanup stale webhook event".to_string()),
    };
    let response: FlagCommentPublic200Response = delete_pending_webhook_event(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
