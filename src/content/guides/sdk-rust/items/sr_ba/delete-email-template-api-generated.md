## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let version: Option<&str> = Some("2025");
    let template_id: String = if let Some(ver) = version {
        format!("welcome-email-{}", ver)
    } else {
        "welcome-email".to_owned()
    };
    let params: DeleteEmailTemplateParams = DeleteEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        id: template_id,
    };
    let _response: ApiEmptyResponse = delete_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---