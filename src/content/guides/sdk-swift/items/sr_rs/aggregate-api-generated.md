Агрегира документе групишући их (ако је groupBy наведен) и примењући више операција. Подржане су различите операције (нпр. sum, countDistinct, avg итд.).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| parentTenantId | string | query | Не |  |
| includeStats | boolean | query | Не |  |

## Одговор

Враћа: [`AggregateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример агрегирања'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било какав проблем, пријавите на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let aggregationRequest = AggregationRequest(query: [QueryPredicate(key: "key_example", value: QueryPredicate_value(), _operator: "_operator_example")], resourceName: "resourceName_example", groupBy: ["groupBy_example"], operations: [AggregationOperation(field: "field_example", op: AggregationOpType(), alias: "alias_example", expandArray: false)], sort: AggregationRequest_sort(dir: "dir_example", field: "field_example")) // AggregationRequest | 
let parentTenantId = "parentTenantId_example" // String |  (опционо)
let includeStats = true // Bool |  (опционо)

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