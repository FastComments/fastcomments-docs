## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| startDate | string | query | No |  |
| questionId | string | query | No |  |
| questionIds | string | query | No |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (optional)
let userId = "userId_example" // String |  (optional)
let startDate = "startDate_example" // String |  (optional)
let questionId = "questionId_example" // String |  (optional)
let questionIds = "questionIds_example" // String |  (optional)
let skip = 987 // Double |  (optional)

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