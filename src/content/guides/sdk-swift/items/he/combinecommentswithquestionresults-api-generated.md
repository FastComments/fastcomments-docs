## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |
| minValue | number | query | No |  |
| maxValue | number | query | No |  |
| limit | number | query | No |  |

## תגובה

מחזיר: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineQuestionResultsWithCommentsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults דוגמה'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הקוד הבא עדיין במצב בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (אופציונלי)
let questionIds = ["inner_example"] // [String] |  (אופציונלי)
let urlId = "urlId_example" // String |  (אופציונלי)
let startDate = Date() // Date |  (אופציונלי)
let forceRecalculate = true // Bool |  (אופציונלי)
let minValue = 987 // Double |  (אופציונלי)
let maxValue = 987 // Double |  (אופציונלי)
let limit = 987 // Double |  (אופציונלי)

DefaultAPI.combineCommentsWithQuestionResults(tenantId: tenantId, options: DefaultAPI.CombineCommentsWithQuestionResultsOptions(questionId: questionId, questionIds: questionIds, urlId: urlId, startDate: startDate, forceRecalculate: forceRecalculate, minValue: minValue, maxValue: maxValue, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]