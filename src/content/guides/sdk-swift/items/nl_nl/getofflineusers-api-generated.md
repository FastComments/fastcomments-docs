Vorige reageerders op de pagina die momenteel NIET online zijn. Gesorteerd op displayName.
Gebruik dit nadat u /users/online hebt uitgeput om een "Leden"-sectie weer te geven.
Cursor-paginering op commenterName: de server doorloopt de gedeeltelijke {tenantId, urlId, commenterName}
index vanaf afterName vooruit via $gt, zonder $skip-kosten.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Pagina-URL-identificatie (gereinigd aan de serverzijde). |
| afterName | string | query | No | Cursor: geef nextAfterName van het vorige antwoord door. |
| afterUserId | string | query | No | Tiebreaker voor de cursor: geef nextAfterUserId van het vorige antwoord door. Vereist wanneer afterName is ingesteld zodat bij gelijke namen geen items wegvallen. |

## Response

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Pagina-URL-identificatie (gereinigd aan de serverzijde).
let afterName = "afterName_example" // String | Cursor: geef nextAfterName van het vorige antwoord door. (optioneel)
let afterUserId = "afterUserId_example" // String | Tiebreaker voor de cursor: geef nextAfterUserId van het vorige antwoord door. Vereist wanneer afterName is ingesteld zodat bij gelijke namen geen items wegvallen. (optioneel)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]