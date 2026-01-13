## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| create_email_template_body | models::CreateEmailTemplateBody | כן |  |

## תגובה

מחזיר: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_email_template_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-create_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateEmailTemplateParams = CreateEmailTemplateParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_email_template_body: models::CreateEmailTemplateBody {
        name: "New Comment Notification".to_string(),
        subject: "New comment on your article".to_string(),
        html_body: "<p>A new comment was posted on <strong>{article_title}</strong>.</p>".to_string(),
        text_body: Some("A new comment was posted on {article_title}.".to_string()),
        from_email: Some("no-reply@acme-news.com".to_string()),
        enabled: Some(true),
    },
};
let response: CreateEmailTemplate200Response = create_email_template(&configuration, params).await?;
[inline-code-end]

---