## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nein |  |
| userId | string | query | Nein |  |
| startDate | string | query | Nein |  |
| questionId | string | query | Nein |  |
| questionIds | string | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## Beispiel

[inline-code-attrs-start title = 'getQuestionResults Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie sich bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (optional)
let userId = "userId_example" // String |  (optional)
let startDate = "startDate_example" // String |  (optional)
let questionId = "questionId_example" // String |  (optional)
let questionIds = "questionIds_example" // String |  (optional)
let skip = 987 // Double |  (optional)

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

---