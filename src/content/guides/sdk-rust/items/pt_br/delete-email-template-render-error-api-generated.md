## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| error_id | String | Sim |  |

## Resposta

Retorna: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Exemplo

[inline-code-attrs-start title = 'delete_email_template_render_error Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteEmailTemplateRenderErrorParams = DeleteEmailTemplateRenderErrorParams {
    tenant_id: String::from("acme-corp-tenant"),
    id: String::from("marketing/newsletter/welcome"),
    error_id: String::from("render_err_2026-06-15-7a3f"),
    request_id: Some(String::from("req-83b2f9a1")),
};

let response: ApiEmptyResponse = delete_email_template_render_error(&configuration, params).await?;
[inline-code-end]

---