## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vraća: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_email_template Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetEmailTemplateParams = GetEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "marketing/welcome_v2".to_string(),
    };
    let template: GetEmailTemplateResponse = get_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---