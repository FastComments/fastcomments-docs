---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |

## Yanıt

Döndürür: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_email_template Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetEmailTemplateParams = GetEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "marketing/welcome_v2".to_string(),
    };
    let template: GetEmailTemplateResponse = get_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---