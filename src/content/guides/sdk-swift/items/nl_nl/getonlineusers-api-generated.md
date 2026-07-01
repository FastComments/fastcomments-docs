Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina URL identifier (server-side opgeschoond). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door van de vorige respons. |
| afterUserId | string | query | Nee | Cursor-tiebreaker: geef nextAfterUserId door van de vorige respons. Vereist wanneer afterName is ingesteld zodat naam-gelijkstanden geen items verliezen. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in beta. Voor elk probleem, rapporteer via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Pagina URL identifier (server-side opgeschoond).
let afterName = "afterName_example" // String | Cursor: geef nextAfterName door van de vorige respons. (optioneel)
let afterUserId = "afterUserId_example" // String | Cursor-tiebreaker: geef nextAfterUserId door van de vorige respons. Vereist wanneer afterName is ingesteld zodat naam-gelijkstanden geen items verliezen. (optioneel)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]