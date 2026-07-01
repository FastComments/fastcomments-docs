## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | query | Tak |  |
| urlId | string | query | Nie |  |
| userId | string | query | Nie |  |
| startDate | string | query | Nie |  |
| questionId | string | query | Nie |  |
| questionIds | string | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResultsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'getQuestionResults Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie jakichkolwiek problemów prosimy zgłaszać je poprzez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (opcjonalny)
let userId = "userId_example" // String |  (opcjonalny)
let startDate = "startDate_example" // String |  (opcjonalny)
let questionId = "questionId_example" // String |  (opcjonalny)
let questionIds = "questionIds_example" // String |  (opcjonalny)
let skip = 987 // Double |  (opcjonalny)

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