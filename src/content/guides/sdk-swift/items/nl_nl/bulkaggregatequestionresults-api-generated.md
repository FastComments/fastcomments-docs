## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| forceRecalculate | boolean | query | Nee |  |

## Response

Retourneert: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BulkAggregateQuestionResults200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let bulkAggregateQuestionResultsRequest = BulkAggregateQuestionResultsRequest(aggregations: [BulkAggregateQuestionItem(aggId: "aggId_example", questionId: "questionId_example", questionIds: ["questionIds_example"], urlId: "urlId_example", timeBucket: AggregateTimeBucket(), startDate: Date())]) // BulkAggregateQuestionResultsRequest | 
let forceRecalculate = true // Bool |  (optioneel)

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