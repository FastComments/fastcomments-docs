## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| skip | f64 | Non |  |

## Réponse

Retourne: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_templates_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_email_templates'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_email_templates() -> Result<GetEmailTemplates200Response, Error> {
    let params: GetEmailTemplatesParams = GetEmailTemplatesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let templates: GetEmailTemplates200Response = get_email_templates(&configuration, params).await?;
    Ok(templates)
}
[inline-code-end]

---