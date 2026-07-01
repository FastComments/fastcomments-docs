## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Yes |  |
| render_email_template_body | models::RenderEmailTemplateBody | Yes |  |
| locale | String | No |  |

## Risposta

Restituisce: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/render_email_template_response.rs)

## Esempio

[inline-code-attrs-start title = 'render_email_template Esempio'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let mut vars = std::collections::HashMap::new();
vars.insert("article_title".to_string(), "Breaking News".to_string());
vars.insert("author".to_string(), "Jane Smith".to_string());

let body = models::RenderEmailTemplateBody {
    template_id: "newsletter".to_string(),
    variables: vars,
};

let params = RenderEmailTemplateParams {
    tenant_id: "acme-corp-tenant".to_string(),
    render_email_template_body: body,
    locale: Some("en-US".to_string()),
};

let response = render_email_template(&configuration, params).await?;
[inline-code-end]