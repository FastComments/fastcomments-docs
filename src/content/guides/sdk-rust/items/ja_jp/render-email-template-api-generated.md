## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| render_email_template_body | models::RenderEmailTemplateBody | はい |  |
| locale | String | いいえ |  |

## レスポンス

戻り値: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/render_email_template_response.rs)

## 例

[inline-code-attrs-start title = 'render_email_template の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let render_body: models::RenderEmailTemplateBody = models::RenderEmailTemplateBody {
        template_id: "notifications/comment_reply".to_string(),
        subject: "Someone replied to your comment".to_string(),
        recipient: "jane.doe@example.com".to_string(),
        variables: std::collections::HashMap::from([
            ("commenter".to_string(), "Alice".to_string()),
            ("post_title".to_string(), "How to Rust".to_string()),
        ]),
    };

    let params: RenderEmailTemplateParams = RenderEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        render_email_template_body: render_body,
        locale: Some("en-US".to_string()),
    };

    let response: RenderEmailTemplateResponse = render_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---