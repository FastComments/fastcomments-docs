## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |

## Resposta

Retorna: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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