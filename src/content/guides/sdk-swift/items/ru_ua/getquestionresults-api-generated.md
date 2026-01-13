## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Нет |  |
| userId | string | query | Нет |  |
| startDate | string | query | Нет |  |
| questionId | string | query | Нет |  |
| questionIds | string | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример getQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бета-версии. По любым проблемам, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (необязательно)
let userId = "userId_example" // String |  (необязательно)
let startDate = "startDate_example" // String |  (необязательно)
let questionId = "questionId_example" // String |  (необязательно)
let questionIds = "questionIds_example" // String |  (необязательно)
let skip = 987 // Double |  (необязательно)

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