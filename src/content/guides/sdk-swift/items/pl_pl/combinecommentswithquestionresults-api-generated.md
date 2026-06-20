## Parametry

| Name | Type | Location | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| questionId | string | query | Nie |  |
| questionIds | array | query | Nie |  |
| urlId | string | query | Nie |  |
| startDate | string | query | Nie |  |
| forceRecalculate | boolean | query | Nie |  |
| minValue | number | query | Nie |  |
| maxValue | number | query | Nie |  |
| limit | number | query | Nie |  |

## Odpowiedź

Zwraca: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineQuestionResultsWithCommentsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemu zgłoś go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (opcjonalne)
let questionIds = ["inner_example"] // [String] |  (opcjonalne)
let urlId = "urlId_example" // String |  (opcjonalne)
let startDate = Date() // Date |  (opcjonalne)
let forceRecalculate = true // Bool |  (opcjonalne)
let minValue = 987 // Double |  (opcjonalne)
let maxValue = 987 // Double |  (opcjonalne)
let limit = 987 // Double |  (opcjonalne)

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