## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| skip | f64 | Не |  |

## Одговор

Враћа: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_email_template_render_errors'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<(), Error> {
    let params: GetEmailTemplateRenderErrorsParams = GetEmailTemplateRenderErrorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email-template".to_string(),
        skip: Some(10.0),
    };
    let response: GetEmailTemplateRenderErrors200Response =
        get_email_template_render_errors(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---