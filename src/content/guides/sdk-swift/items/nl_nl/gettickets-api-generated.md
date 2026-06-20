## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nee |  |
| state | number | query | Nee |  |
| skip | number | query | Nee |  |
| limit | number | query | Nee |  |

## Respons

Retourneert: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicketsResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getTickets Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Bij problemen, meld ze via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (optioneel)
let state = 987 // Double |  (optioneel)
let skip = 987 // Double |  (optioneel)
let limit = 987 // Double |  (optioneel)

DefaultAPI.getTickets(tenantId: tenantId, userId: userId, state: state, skip: skip, limit: limit) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]