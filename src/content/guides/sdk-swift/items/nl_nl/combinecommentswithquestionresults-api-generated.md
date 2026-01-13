## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nee |  |
| questionIds | array | query | Nee |  |
| urlId | string | query | Nee |  |
| startDate | string | query | Nee |  |
| forceRecalculate | boolean | query | Nee |  |
| minValue | number | query | Nee |  |
| maxValue | number | query | Nee |  |
| limit | number | query | Nee |  |

## Respons

Retourneert: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineCommentsWithQuestionResults200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (optioneel)
let questionIds = ["inner_example"] // [String] |  (optioneel)
let urlId = "urlId_example" // String |  (optioneel)
let startDate = Date() // Date |  (optioneel)
let forceRecalculate = true // Bool |  (optioneel)
let minValue = 987 // Double |  (optioneel)
let maxValue = 987 // Double |  (optioneel)
let limit = 987 // Double |  (optioneel)

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