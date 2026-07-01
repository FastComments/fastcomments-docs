Агрегує документи шляхом групування (якщо передано **groupBy**) та застосуванням декількох операцій.  
Підтримуються різні операції (наприклад, **sum**, **countDistinct**, **avg** тощо).

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|--------------|------|
| tenant_id | String | Yes |  |
| aggregation_request | models::AggregationRequest | Yes |  |
| parent_tenant_id | String | No |  |
| include_stats | bool | No |  |

## Відповідь

Повертає: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад агрегування'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let aggregation_request = models::AggregationRequest::default();
    let params = AggregateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        aggregation_request,
        parent_tenant_id: Some("global-tenant".to_string()),
        include_stats: Some(true),
    };
    let _response = aggregate(&config, params).await?;
    Ok(())
}
[inline-code-end]