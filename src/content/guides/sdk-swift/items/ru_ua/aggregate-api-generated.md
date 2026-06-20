Агрегирует документы, группируя их (если указан groupBy) и применяя несколько операций.
Поддерживаются разные операции (например, sum, countDistinct, avg и т.д.).

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| parentTenantId | string | query | Нет |  |
| includeStats | boolean | query | Нет |  |

## Ответ

Возвращает: [`AggregateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример aggregate'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все ещё в бета. По любым проблемам, пожалуйста, сообщайте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let aggregationRequest = AggregationRequest(query: [QueryPredicate(key: "key_example", value: QueryPredicate_value(), _operator: "_operator_example")], resourceName: "resourceName_example", groupBy: ["groupBy_example"], operations: [AggregationOperation(field: "field_example", op: AggregationOpType(), alias: "alias_example", expandArray: false)], sort: AggregationRequest_sort(dir: "dir_example", field: "field_example")) // AggregationRequest | 
let parentTenantId = "parentTenantId_example" // String |  (необязательно)
let includeStats = true // Bool |  (необязательно)

DefaultAPI.aggregate(tenantId: tenantId, aggregationRequest: aggregationRequest, parentTenantId: parentTenantId, includeStats: includeStats) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]