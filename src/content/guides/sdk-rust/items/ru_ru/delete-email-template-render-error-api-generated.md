## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| error_id | String | Да |  |

## Ответ

Возвращает: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'delete_email_template_render_error Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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