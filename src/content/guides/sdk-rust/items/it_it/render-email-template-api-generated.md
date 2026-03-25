## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| render_email_template_body | models::RenderEmailTemplateBody | Sì |  |
| locale | String | No |  |

## Risposta

Restituisce: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/render_email_template_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di render_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<RenderEmailTemplate200Response, Error> {
    let body = models::RenderEmailTemplateBody {
        template_key: "welcome_email".to_string(),
        subject: "Welcome to Acme News".to_string(),
        from_address: "noreply@acme.com".to_string(),
        placeholders: std::collections::HashMap::from([
            ("user_name".to_string(), "Jane Doe".to_string()),
            ("article_title".to_string(), "Breaking News: Rust Adoption Soars".to_string()),
        ]),
    };
    let params = RenderEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        render_email_template_body: body,
        locale: Some("en-US".to_string()),
    };
    let rendered: RenderEmailTemplate200Response = render_email_template(configuration, params).await?;
    Ok(rendered)
}
[inline-code-end]

---