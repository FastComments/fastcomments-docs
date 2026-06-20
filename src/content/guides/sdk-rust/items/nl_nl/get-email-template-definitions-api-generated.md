## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |

## Antwoord

Retourneert: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_definitions_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_email_template_definitions Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_templates() -> Result<(), Error> {
    let params: GetEmailTemplateDefinitionsParams = GetEmailTemplateDefinitionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        locale: Some("en-US".to_string()),
    };
    let response: GetEmailTemplateDefinitionsResponse =
        get_email_template_definitions(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---