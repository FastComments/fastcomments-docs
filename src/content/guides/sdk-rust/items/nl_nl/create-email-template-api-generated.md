## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_email_template_body | models::CreateEmailTemplateBody | Ja |  |

## Response

Retourneert: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_email_template_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'create_email_template Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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