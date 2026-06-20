## Parâmetros

| Name | Type | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| skip | f64 | Não |  |

## Resposta

Retorna: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_email_template_render_errors'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetEmailTemplateRenderErrorsParams = GetEmailTemplateRenderErrorsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "welcome-email-v2".to_string(),
    skip: Some(10.0),
};
let response: GetEmailTemplateRenderErrorsResponse = get_email_template_render_errors(&configuration, params).await?;
[inline-code-end]

---