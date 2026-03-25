---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vraća: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_email_template Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let tenant_id: String = "acme-corp-tenant".to_string();
    let template_id: String = "welcome-new-subscriber".to_string();
    let params: GetEmailTemplateParams = GetEmailTemplateParams {
        tenant_id,
        id: template_id,
        locale: Some("en-US".to_string()),
    };
    let template: GetEmailTemplate200Response = get_email_template(&configuration, params).await?;
    let _ = template;
    Ok(())
}
[inline-code-end]

---