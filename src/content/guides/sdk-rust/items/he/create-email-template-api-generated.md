---
## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| create_email_template_body | models::CreateEmailTemplateBody | כן |  |

## תגובה

מחזיר: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_email_template_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-create_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateEmailTemplateParams = CreateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".into(),
        create_email_template_body: models::CreateEmailTemplateBody {
            name: "Article Comment Notification".into(),
            subject: "New comment on your article".into(),
            body_html: "<p>Hi \{{recipient_name}},</p><p>\{{comment_author}} left a comment on your article \"\{{article_title}}\".</p>".into(),
            from_name: Some("Acme News".into()),
            from_email: Some("notifications@acme.example".into()),
            reply_to: Some("no-reply@acme.example".into()),
            enabled: Some(true),
            tags: Some(vec!["comments".into(), "notifications".into()]),
        },
    };
    let response: CreateEmailTemplate200Response = create_email_template(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---