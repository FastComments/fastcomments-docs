## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_notification_count Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_delete_notification_count() -> Result<FlagCommentPublic200Response, Error> {
    let tenant_scope: Option<String> = Some("acme-corp-tenant".to_string());
    let params: DeleteNotificationCountParams = DeleteNotificationCountParams {
        tenant_id: tenant_scope.unwrap(),
        id: "notification-9f2b3a".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_notification_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
