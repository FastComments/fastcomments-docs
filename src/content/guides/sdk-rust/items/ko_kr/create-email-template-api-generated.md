## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_email_template_body | models::CreateEmailTemplateBody | Yes |  |

## 응답

Returns: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_email_template_response.rs)

## 예시

[inline-code-attrs-start title = 'create_email_template 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = CreateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_email_template_body: models::CreateEmailTemplateBody {
            name: "welcome".to_string(),
            subject: "Welcome to Acme".to_string(),
            html_content: "<h1>Welcome</h1>".to_string(),
            plain_text_content: Some("Welcome to Acme".to_string()),
        },
    };
    let _response = create_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]