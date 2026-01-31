## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_email_template_body | models::CreateEmailTemplateBody | Yes |  |

## Response

Returns: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_email_template_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_email_template Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateEmailTemplateParams = CreateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_email_template_body: models::CreateEmailTemplateBody {
            name: "Weekly Newsletter".to_string(),
            subject: "Acme Corp — Weekly Product Updates".to_string(),
            from_name: Some("Acme Corp".to_string()),
            from_email: "newsletter@acme-corp.com".to_string(),
            html_content: "<h1>{{headline}}</h1><p>Hello {{first_name}},</p><p>Latest updates inside.</p>".to_string(),
            plain_text: Some("Hello {{first_name}},\n\nLatest updates inside.".to_string()),
            enabled: Some(true),
            tags: Some(vec!["newsletter".to_string(), "weekly".to_string()]),
        },
    };

    let response: CreateEmailTemplate200Response = create_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
