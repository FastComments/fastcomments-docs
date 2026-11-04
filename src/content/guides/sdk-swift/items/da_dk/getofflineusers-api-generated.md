Tidligere kommentatorer på siden, som IKKE er online i øjeblikket. Sorteret efter displayName.
Brug denne efter du har opbrugt /users/online for at rendere et "Members"-afsnit.
Cursor-paginering på commenterName: serveren gennemløber det partielle {tenantId, urlId, commenterName}
indeks fra afterName fremad via $gt, uden $skip-omkostninger.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Sidens URL-identifikator (renset på serversiden). |
| afterName | string | query | Nej | Cursor: send nextAfterName fra det forrige svar. |
| afterUserId | string | query | Nej | Cursor tiebreaker: send nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så navneties ikke mister poster. |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Sidens URL-identifikator (renset på serversiden).
let afterName = "afterName_example" // String | Cursor: send nextAfterName fra det forrige svar. (optional)
let afterUserId = "afterUserId_example" // String | Cursor tiebreaker: send nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så navneties ikke mister poster. (optional)

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