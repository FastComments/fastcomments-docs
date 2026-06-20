## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenant_id | String | Oui |  |
| render_email_template_body | models::RenderEmailTemplateBody | Oui |  |
| locale | String | Non |  |

## Réponse

Retourne : [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/render_email_template_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de render_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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