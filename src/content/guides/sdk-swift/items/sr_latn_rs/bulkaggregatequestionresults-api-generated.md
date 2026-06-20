## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| forceRecalculate | boolean | query | Ne |  |

## Odgovor

Vraća: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BulkAggregateQuestionResultsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer bulkAggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek u beta fazi. Za bilo koji problem, prijavite preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let bulkAggregateQuestionResultsRequest = BulkAggregateQuestionResultsRequest(aggregations: [BulkAggregateQuestionItem(aggId: "aggId_example", questionId: "questionId_example", questionIds: ["questionIds_example"], urlId: "urlId_example", timeBucket: AggregateTimeBucket(), startDate: Date())]) // BulkAggregateQuestionResultsRequest | 
let forceRecalculate = true // Bool |  (neobavezno)

DefaultAPI.bulkAggregateQuestionResults(tenantId: tenantId, bulkAggregateQuestionResultsRequest: bulkAggregateQuestionResultsRequest, forceRecalculate: forceRecalculate) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]