## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| create_email_template_body | models::CreateEmailTemplateBody | כן |  |

## תגובה

מחזיר: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_email_template_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-create_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = CreateEmailTemplateParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_email_template_body: models::CreateEmailTemplateBody {
        name: "Weekly Newsletter".to_string(),
        slug: Some("news/weekly".to_string()),
        subject: Some("Acme Corp — Weekly Updates".to_string()),
        html_body: Some("<h1>Acme Weekly</h1><p>Top stories this week...</p>".to_string()),
        text_body: Some("Acme Weekly — Top stories this week...".to_string()),
        from_email: Some("newsletter@acme.com".to_string()),
        reply_to: Some("support@acme.com".to_string()),
        description: Some("Template used for the weekly customer newsletter".to_string()),
        is_active: Some(true),
    },
};
let created: CreateEmailTemplateResponse = create_email_template(&configuration, params).await?;
[inline-code-end]

---