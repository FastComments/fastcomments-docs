## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример delete_pending_webhook_event'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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