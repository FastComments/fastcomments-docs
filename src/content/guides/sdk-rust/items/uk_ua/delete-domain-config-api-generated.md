## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| domain | String | Yes |  |

## Відповідь

Повертає: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_domain_config_response.rs)

## Приклад

[inline-code-attrs-start title = 'delete_domain_config Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_domain_config() -> Result<DeleteDomainConfigResponse, Error> {
    let params = DeleteDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        domain: "news/acme-corp".to_owned(),
        force: Some(true),
    };
    let response: DeleteDomainConfigResponse = delete_domain_config(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---