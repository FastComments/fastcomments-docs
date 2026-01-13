## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Одговор

Враћа: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_200_response.rs)

## Пример

[inline-code-attrs-start title = 'get_email_template Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_template() -> Result<GetEmailTemplate200Response, Error> {
    let params: GetEmailTemplateParams = GetEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email-template".to_string(),
    };
    let template: GetEmailTemplate200Response = get_email_template(&configuration, params).await?;
    Ok(template)
}
[inline-code-end]

---