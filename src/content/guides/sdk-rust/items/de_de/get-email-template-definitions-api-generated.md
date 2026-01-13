---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |

## Antwort

Gibt zurück: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_definitions_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_email_template_definitions Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let config: configuration::Configuration = configuration::Configuration::default();
    let params: GetEmailTemplateDefinitionsParams = GetEmailTemplateDefinitionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        locale: Some("en-US".to_string()),
        include_inactive: Some(false),
    };
    let resp: GetEmailTemplateDefinitions200Response = get_email_template_definitions(&config, params).await?;
    let _definitions = resp;
    Ok(())
}
[inline-code-end]

---