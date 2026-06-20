Momenteel online kijkers van een pagina: personen wiens websocket-sessie op dit moment op de pagina geabonneerd is.
Geeft anonCount + totalCount terug (kamer-brede abonnees, inclusief anonieme kijkers die we niet afzonderlijk opsommen).

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina-URL-identificatie (schoongemaakt aan de serverzijde). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door uit de vorige respons. |
| afterUserId | string | query | Nee | Cursor-tiebreaker: geef nextAfterUserId door uit de vorige respons. Vereist wanneer afterName is ingesteld zodat gelijke namen niet leiden tot het wegvallen van vermeldingen. |

## Response

Geeft terug: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codesamples zijn nog in bèta. Bij problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Pagina-URL-identificatie (schoongemaakt aan de serverzijde).
let afterName = "afterName_example" // String | Cursor: geef nextAfterName door uit de vorige respons. (optioneel)
let afterUserId = "afterUserId_example" // String | Cursor-tiebreaker: geef nextAfterUserId door uit de vorige respons. Vereist wanneer afterName is ingesteld zodat gelijke namen niet leiden tot het wegvallen van vermeldingen. (optioneel)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]