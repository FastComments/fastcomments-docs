## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| questionId | string | query | לא |  |
| questionIds | array | query | לא |  |
| urlId | string | query | לא |  |
| startDate | string | query | לא |  |
| forceRecalculate | boolean | query | לא |  |
| minValue | number | query | לא |  |
| maxValue | number | query | לא |  |
| limit | number | query | לא |  |

## תגובה

מחזיר: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineCommentsWithQuestionResults200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-combineCommentsWithQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמי הקוד שלהלן עדיין בבטא. אם יש בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
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

DefaultAPI.combineCommentsWithQuestionResults(tenantId: tenantId, questionId: questionId, questionIds: questionIds, urlId: urlId, startDate: startDate, forceRecalculate: forceRecalculate, minValue: minValue, maxValue: maxValue, limit: limit) { (response, error) in
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