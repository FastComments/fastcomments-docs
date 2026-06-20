## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nee |  |
| userId | string | query | Nee |  |
| startDate | string | query | Nee |  |
| questionId | string | query | Nee |  |
| questionIds | string | query | Nee |  |
| skip | number | query | Nee |  |

## Respons

Retourneert: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResultsResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getQuestionResults Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Als er een probleem is, meld dit dan via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (optioneel)
let userId = "userId_example" // String |  (optioneel)
let startDate = "startDate_example" // String |  (optioneel)
let questionId = "questionId_example" // String |  (optioneel)
let questionIds = "questionIds_example" // String |  (optioneel)
let skip = 987 // Double |  (optioneel)

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