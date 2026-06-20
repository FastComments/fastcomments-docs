## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Одговор

Враћа: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'delete_email_template Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let version: Option<&str> = Some("2025");
    let template_id: String = if let Some(ver) = version {
        format!("welcome-email-{}", ver)
    } else {
        "welcome-email".to_owned()
    };
    let params: DeleteEmailTemplateParams = DeleteEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        id: template_id,
    };
    let _response: ApiEmptyResponse = delete_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---