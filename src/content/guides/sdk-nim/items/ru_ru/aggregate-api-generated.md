Агрегирует документы, группируя их (если указан параметр groupBy), и применяя несколько операций.  
Поддерживаются различные операции (например, sum, countDistinct, avg и т.д.).

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| aggregationRequest | AggregationRequest | Нет |  |
| options | AggregateOptions | Нет |  |

## Ответ

Возвращает: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Пример

[inline-code-attrs-start title = 'aggregate Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]

---