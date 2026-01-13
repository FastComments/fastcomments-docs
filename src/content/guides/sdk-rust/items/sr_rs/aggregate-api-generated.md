Агрегира документа по груписању (ако је groupBy наведен) и примени више операција.
Подржане су различите операције (нпр. sum, countDistinct, avg итд.).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| aggregation_request | models::AggregationRequest | Да |  |
| parent_tenant_id | String | Не |  |
| include_stats | bool | Не |  |

## Одговор

Враћа: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---