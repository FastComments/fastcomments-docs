## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Не |  |
| userId | string | query | Не |  |
| startDate | string | query | Не |  |
| questionId | string | query | Не |  |
| questionIds | string | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни кодове все още са в бета. За проблеми, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (незадължително)
let userId = "userId_example" // String |  (незадължително)
let startDate = "startDate_example" // String |  (незадължително)
let questionId = "questionId_example" // String |  (незадължително)
let questionIds = "questionIds_example" // String |  (незадължително)
let skip = 987 // Double |  (незадължително)

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