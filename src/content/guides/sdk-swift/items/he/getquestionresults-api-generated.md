## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | לא |  |
| userId | string | query | לא |  |
| startDate | string | query | לא |  |
| questionId | string | query | לא |  |
| questionIds | string | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בבטא. לכל בעיה, נא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (אופציונלי)
let userId = "userId_example" // String |  (אופציונלי)
let startDate = "startDate_example" // String |  (אופציונלי)
let questionId = "questionId_example" // String |  (אופציונלי)
let questionIds = "questionIds_example" // String |  (אופציונלי)
let skip = 987 // Double |  (אופציונלי)

DefaultAPI.getQuestionResults(tenantId: tenantId, urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]