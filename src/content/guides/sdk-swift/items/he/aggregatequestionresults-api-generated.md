## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| questionId | string | query | לא |  |
| questionIds | array | query | לא |  |
| urlId | string | query | לא |  |
| timeBucket | string | query | לא |  |
| startDate | string | query | לא |  |
| forceRecalculate | boolean | query | לא |  |

## Response

מחזיר: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResultsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של aggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדגימות הקודיות הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח באמצעות http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (optional)
let questionIds = ["inner_example"] // [String] |  (optional)
let urlId = "urlId_example" // String |  (optional)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (optional)
let startDate = Date() // Date |  (optional)
let forceRecalculate = true // Bool |  (optional)

DefaultAPI.aggregateQuestionResults(tenantId: tenantId, options: DefaultAPI.AggregateQuestionResultsOptions(questionId: questionId, questionIds: questionIds, urlId: urlId, timeBucket: timeBucket, startDate: startDate, forceRecalculate: forceRecalculate)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]