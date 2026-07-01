## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_email_template_body | models::UpdateEmailTemplateBody | Yes |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primer

[inline-code-attrs-start title = 'Пример update_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = UpdateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email".to_string(),
        update_email_template_body: models::UpdateEmailTemplateBody {
            subject: Some("Welcome to Acme Corp".to_string()),
            body_html: Some("<p>Hello, \{{user.name}}!</p>".to_string()),
            body_text: None,
        },
    };
    let _ = update_email_template(&config, params).await?;
    Ok(())
}
[inline-code-end]