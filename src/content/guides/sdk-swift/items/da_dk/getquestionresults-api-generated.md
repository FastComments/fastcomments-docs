## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nej |  |
| userId | string | query | Nej |  |
| startDate | string | query | Nej |  |
| questionId | string | query | Nej |  |
| questionIds | string | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'getQuestionResults Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (valgfri)
let userId = "userId_example" // String |  (valgfri)
let startDate = "startDate_example" // String |  (valgfri)
let questionId = "questionId_example" // String |  (valgfri)
let questionIds = "questionIds_example" // String |  (valgfri)
let skip = 987 // Double |  (valgfri)

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