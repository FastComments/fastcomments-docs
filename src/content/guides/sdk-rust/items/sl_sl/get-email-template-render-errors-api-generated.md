## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| skip | f64 | Ne |  |

## Odgovor

Vrne: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_email_template_render_errors'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetEmailTemplateRenderErrorsParams = GetEmailTemplateRenderErrorsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "welcome-email-v2".to_string(),
    skip: Some(10.0),
};
let response: GetEmailTemplateRenderErrorsResponse = get_email_template_render_errors(&configuration, params).await?;
[inline-code-end]

---