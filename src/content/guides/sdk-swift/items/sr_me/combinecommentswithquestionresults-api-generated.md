## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| questionId | string | query | Ne |  |
| questionIds | array | query | Ne |  |
| urlId | string | query | Ne |  |
| startDate | string | query | Ne |  |
| forceRecalculate | boolean | query | Ne |  |
| minValue | number | query | Ne |  |
| maxValue | number | query | Ne |  |
| limit | number | query | Ne |  |

## Odgovor

Vraća: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineQuestionResultsWithCommentsResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer combineCommentsWithQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su i dalje u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (opcionalno)
let questionIds = ["inner_example"] // [String] |  (opcionalno)
let urlId = "urlId_example" // String |  (opcionalno)
let startDate = Date() // Date |  (opcionalno)
let forceRecalculate = true // Bool |  (opcionalno)
let minValue = 987 // Double |  (opcionalno)
let maxValue = 987 // Double |  (opcionalno)
let limit = 987 // Double |  (opcionalno)

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