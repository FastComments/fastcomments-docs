## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Eksempel

[inline-code-attrs-start title = 'delete_pending_webhook_event Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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