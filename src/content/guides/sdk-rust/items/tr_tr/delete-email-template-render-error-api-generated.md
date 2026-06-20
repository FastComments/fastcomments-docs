---
## Parametreler

| Name | Type | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |
| error_id | String | Evet |  |

## Yanıt

Döndürür: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Örnek

[inline-code-attrs-start title = 'delete_email_template_render_error Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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