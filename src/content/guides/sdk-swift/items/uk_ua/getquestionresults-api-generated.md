## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| urlId | string | query | Ні |  |
| userId | string | query | Ні |  |
| startDate | string | query | Ні |  |
| questionId | string | query | Ні |  |
| questionIds | string | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResultsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getQuestionResults Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду ще є бета-версією. Якщо виникнуть проблеми, будь ласка, повідомте їх за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (необов'язковий)
let userId = "userId_example" // String |  (необов'язковий)
let startDate = "startDate_example" // String |  (необов'язковий)
let questionId = "questionId_example" // String |  (необов'язковий)
let questionIds = "questionIds_example" // String |  (необов'язковий)
let skip = 987 // Double |  (необов'язковий)

DefaultAPI.getQuestionResults(tenantId: tenantId, options: DefaultAPI.GetQuestionResultsOptions(urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip)) { (response, error) in
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