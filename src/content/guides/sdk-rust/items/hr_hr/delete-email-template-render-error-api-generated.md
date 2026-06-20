## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| error_id | String | Da |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_email_template_render_error'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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