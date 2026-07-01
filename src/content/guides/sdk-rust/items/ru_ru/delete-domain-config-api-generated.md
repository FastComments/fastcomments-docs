## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenant_id | String | Да |  |
| domain | String | Да |  |

## Ответ

Возвращает: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_domain_config_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример delete_domain_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = DeleteDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain: "news/article".to_string(),
    };
    let _response: DeleteDomainConfigResponse = delete_domain_config(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---