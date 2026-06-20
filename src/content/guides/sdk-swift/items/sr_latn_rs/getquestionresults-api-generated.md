## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne |  |
| userId | string | query | Ne |  |
| startDate | string | query | Ne |  |
| questionId | string | query | Ne |  |
| questionIds | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResultsResponse.swift)

## Primer

[inline-code-attrs-start title = 'getQuestionResults Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (opcionalno)
let userId = "userId_example" // String |  (opcionalno)
let startDate = "startDate_example" // String |  (opcionalno)
let questionId = "questionId_example" // String |  (opcionalno)
let questionIds = "questionIds_example" // String |  (opcionalno)
let skip = 987 // Double |  (opcionalno)

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