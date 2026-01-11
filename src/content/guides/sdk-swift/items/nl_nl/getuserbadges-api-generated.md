## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nee |  |
| badgeId | string | query | Nee |  |
| type | number | query | Nee |  |
| displayedOnComments | boolean | query | Nee |  |
| limit | number | query | Nee |  |
| skip | number | query | Nee |  |

## Respons

Retourneert: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserBadges200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getUserBadges Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden bevinden zich nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (optioneel)
let badgeId = "badgeId_example" // String |  (optioneel)
let type = 987 // Double |  (optioneel)
let displayedOnComments = true // Bool |  (optioneel)
let limit = 987 // Double |  (optioneel)
let skip = 987 // Double |  (optioneel)

DefaultAPI.getUserBadges(tenantId: tenantId, userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]