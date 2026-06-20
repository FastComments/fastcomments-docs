## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Не |  |
| userId | string | query | Не |  |
| startDate | string | query | Не |  |
| questionId | string | query | Не |  |
| questionIds | string | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResultsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getQuestionResults Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бети. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (необавезно)
let userId = "userId_example" // String |  (необавезно)
let startDate = "startDate_example" // String |  (необавезно)
let questionId = "questionId_example" // String |  (необавезно)
let questionIds = "questionIds_example" // String |  (необавезно)
let skip = 987 // Double |  (необавезно)

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