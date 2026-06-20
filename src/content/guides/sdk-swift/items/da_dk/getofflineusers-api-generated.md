Tidligere kommentatorer på siden, som IKKE er online lige nu. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at gengive en "Members"-sektion.
Cursor-paginering på commenterName: serveren går gennem den partielle {tenantId, urlId, commenterName}-indeks fra afterName fremad via $gt, ingen $skip-omkostning.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | sti | Ja |  |
| urlId | string | forespørgsel | Ja | Side-URL-identifikator (renses på serversiden). |
| afterName | string | forespørgsel | Nej | Cursor: angiv nextAfterName fra det forrige svar. |
| afterUserId | string | forespørgsel | Nej | Tiebreaker for cursor: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er sat, så navneligheder ikke medfører, at poster udelades. |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Side-URL-identifikator (renses på serversiden).
let afterName = "afterName_example" // String | Cursor: angiv nextAfterName fra det forrige svar. (valgfri)
let afterUserId = "afterUserId_example" // String | Tiebreaker for cursor: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er sat, så navneligheder ikke medfører, at poster udelades. (valgfri)

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