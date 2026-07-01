---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenant_id | String | Да |  |

## Отговор

Връща: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_definitions_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_email_template_definitions'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetEmailTemplateDefinitionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
    };
    let _response = get_email_template_definitions(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---