## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
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

[inline-code-attrs-start title = 'Przykład combineCommentsWithQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W razie jakichkolwiek problemów, proszę zgłaszać je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (opcjonalny)
let questionIds = ["inner_example"] // [String] |  (opcjonalny)
let urlId = "urlId_example" // String |  (opcjonalny)
let startDate = Date() // Date |  (opcjonalny)
let forceRecalculate = true // Bool |  (opcjonalny)
let minValue = 987 // Double |  (opcjonalny)
let maxValue = 987 // Double |  (opcjonalny)
let limit = 987 // Double |  (opcjonalny)

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