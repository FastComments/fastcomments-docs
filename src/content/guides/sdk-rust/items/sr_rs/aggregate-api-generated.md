Агрегира документе груписањем (ако је наведен groupBy) и примењивањем више операција. Подржане су различите операције (нпр. sum, countDistinct, avg, итд.).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| aggregation_request | models::AggregationRequest | Да |  |
| parent_tenant_id | String | Не |  |
| include_stats | bool | Не |  |

## Одговор

Враћа: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Пример

[inline-code-attrs-start title = 'пример агрегирања'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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