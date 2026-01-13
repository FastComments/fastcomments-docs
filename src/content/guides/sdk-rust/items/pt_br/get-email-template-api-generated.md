## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Resposta

Retorna: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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