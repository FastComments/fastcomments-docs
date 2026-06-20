## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Yanıt

Döndürür: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Örnek

[inline-code-attrs-start title = 'delete_email_template Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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