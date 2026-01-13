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

Враћа: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## Пример

[inline-code-attrs-start title = 'getQuestionResults пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример(и) кода су још у бета фази. За било који проблем пријавите на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (опционо)
let userId = "userId_example" // String |  (опционо)
let startDate = "startDate_example" // String |  (опционо)
let questionId = "questionId_example" // String |  (опционо)
let questionIds = "questionIds_example" // String |  (опционо)
let skip = 987 // Double |  (опционо)

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