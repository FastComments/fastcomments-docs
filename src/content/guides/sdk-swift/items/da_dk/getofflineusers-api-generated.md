Past commenters on the page who are NOT currently online. Sorted by displayName.  
Tidligere kommentatorer på siden, der IKKE er online i øjeblikket. Sorteret efter displayName.

Use this after exhausting /users/online to render a "Members" section.  
Brug dette efter at have udtømt /users/online for at gengive en "Members"-sektion.

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Cursor-paginering på commenterName: serveren gennemløber den delvise {tenantId, urlId, commenterName} indeks fra afterName fremad via $gt, uden $skip-omkostning.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side-URL-identifikator (rengjort på serveren). |
| afterName | string | query | No | Cursor: send nextAfterName fra det foregående svar. |
| afterUserId | string | query | No | Cursor-tiebreaker: send nextAfterUserId fra det foregående svar. Påkrævet når afterName er sat, så navneties ikke udelader poster. |

## Response

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. Ved eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Side-URL-identifikator (rengjort på serveren).
let afterName = "afterName_example" // String | Cursor: send nextAfterName fra det foregående svar. (valgfri)
let afterUserId = "afterUserId_example" // String | Cursor-tiebreaker: send nextAfterUserId fra det foregående svar. Påkrævet når afterName er sat, så navneties ikke udelader poster. (valgfri)

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