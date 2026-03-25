## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'delete_pending_webhook_event 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<FlagCommentPublic200Response, Error> {
    let audit_note: Option<String> = Some("removed duplicate webhook event".to_string());
    let params: DeletePendingWebhookEventParams = DeletePendingWebhookEventParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "9f8b7a6c-1234-4b8d-9c3a-0e1f2d3c4b5a".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_pending_webhook_event(&configuration, params).await?;
    let _ = audit_note;
    Ok(response)
}
[inline-code-end]

---