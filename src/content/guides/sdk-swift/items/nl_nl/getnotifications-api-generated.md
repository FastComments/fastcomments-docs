## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nee |  |
| urlId | string | query | Nee |  |
| fromCommentId | string | query | Nee |  |
| viewed | boolean | query | Nee |  |
| type | string | query | Nee |  |
| skip | number | query | Nee |  |

## Respons

Retourneert: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotifications200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getNotifications Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Bij problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (optioneel)
let urlId = "urlId_example" // String |  (optioneel)
let fromCommentId = "fromCommentId_example" // String |  (optioneel)
let viewed = true // Bool |  (optioneel)
let type = "type_example" // String |  (optioneel)
let skip = 987 // Double |  (optioneel)

DefaultAPI.getNotifications(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]