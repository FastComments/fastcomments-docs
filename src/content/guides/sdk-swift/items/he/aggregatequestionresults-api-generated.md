## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| questionId | string | query | לא |  |
| questionIds | array | query | לא |  |
| urlId | string | query | לא |  |
| timeBucket | string | query | לא |  |
| startDate | string | query | לא |  |
| forceRecalculate | boolean | query | לא |  |

## תגובה

מחזיר: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResults200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-aggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. בכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (אופציונלי)
let questionIds = ["inner_example"] // [String] |  (אופציונלי)
let urlId = "urlId_example" // String |  (אופציונלי)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (אופציונלי)
let startDate = Date() // Date |  (אופציונלי)
let forceRecalculate = true // Bool |  (אופציונלי)

DefaultAPI.aggregateQuestionResults(tenantId: tenantId, questionId: questionId, questionIds: questionIds, urlId: urlId, timeBucket: timeBucket, startDate: startDate, forceRecalculate: forceRecalculate) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---