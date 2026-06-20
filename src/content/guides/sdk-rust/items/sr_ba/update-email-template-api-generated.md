## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| update_email_template_body | models::UpdateEmailTemplateBody | Da |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primjer

[inline-code-attrs-start title = 'update_email_template Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let params: UpdateEmailTemplateParams = UpdateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "weekly-newsletter".to_string(),
        update_email_template_body: models::UpdateEmailTemplateBody {
            name: Some("Weekly Newsletter".to_string()),
            subject: Some("Your Weekly Acme Updates".to_string()),
            html: Some("<h1>Acme News</h1><p>Latest product and engineering updates.</p>".to_string()),
            plain_text: Some("Acme News - Latest product and engineering updates.".to_string()),
            enabled: Some(true),
            sender_name: Some("Acme Team".to_string()),
            sender_email: Some("newsletter@acme.com".to_string()),
            locale: Some("en-US".to_string()),
        },
    };
    update_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---