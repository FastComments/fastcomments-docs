Eerdere commenters op de pagina die momenteel NIET online zijn. Gesorteerd op displayName.
Gebruik dit na het doorlopen van /users/online om een "Leden"-sectie weer te geven.
Cursorpaginering op commenterName: de server loopt via $gt door de partiële {tenantId, urlId, commenterName}
index vanaf afterName vooruit, zonder $skip-kosten.

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina-URL-identificatie (serverzijde opgeschoond). |
| afterName | string | query | Nee | Cursor: geef nextAfterName uit het vorige antwoord door. |
| afterUserId | string | query | Nee | Cursor-tiebreaker: geef nextAfterUserId uit het vorige antwoord door. Verplicht wanneer afterName is ingesteld, zodat gelijke namen geen items verliezen. |

## Response

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Pagina-URL-identificatie (serverzijde opgeschoond).
let afterName = "afterName_example" // String | Cursor: geef nextAfterName uit het vorige antwoord door. (optional)
let afterUserId = "afterUserId_example" // String | Cursor-tiebreaker: geef nextAfterUserId uit het vorige antwoord door. Verplicht wanneer afterName is ingesteld, zodat gelijke namen geen items verliezen. (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]