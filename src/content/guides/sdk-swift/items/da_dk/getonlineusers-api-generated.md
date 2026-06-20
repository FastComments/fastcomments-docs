Aktuelt online-seere af en side: personer, hvis websocket-session er abonneret på siden lige nu.
Returnerer anonCount + totalCount (abonnenter på hele rummet, inklusive anonyme seere, som vi ikke opregner).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side-URL-identifikator (renset på serversiden). |
| afterName | string | query | No | Cursor: angiv nextAfterName fra det forrige svar. |
| afterUserId | string | query | No | Cursor-tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så navne-kollisioner ikke udelader poster. |

## Response

Returnerer: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getOnlineUsers Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Side-URL-identifikator (renset på serversiden).
let afterName = "afterName_example" // String | Cursor: angiv nextAfterName fra det forrige svar. (valgfri)
let afterUserId = "afterUserId_example" // String | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så navne-kollisioner ikke udelader poster. (valgfri)

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