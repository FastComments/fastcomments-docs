## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |

## Svar

Returnerer: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_definitions_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_email_template_definitions Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetEmailTemplateDefinitionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
    };
    let _response = get_email_template_definitions(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---