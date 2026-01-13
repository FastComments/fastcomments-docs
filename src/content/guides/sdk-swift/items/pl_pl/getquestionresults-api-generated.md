## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| urlId | string | query | Nie |  |
| userId | string | query | Nie |  |
| startDate | string | query | Nie |  |
| questionId | string | query | Nie |  |
| questionIds | string | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w fazie beta. W przypadku problemu, zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (opcjonalne)
let userId = "userId_example" // String |  (opcjonalne)
let startDate = "startDate_example" // String |  (opcjonalne)
let questionId = "questionId_example" // String |  (opcjonalne)
let questionIds = "questionIds_example" // String |  (opcjonalne)
let skip = 987 // Double |  (opcjonalne)

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