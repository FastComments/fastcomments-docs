Aktuelt online‑visere af en side: personer, hvis websocketsession er abonneret på siden lige nu.  
Returnerer anonCount + totalCount (rum‑omfattende abonnenter, inklusive anonyme seere, som vi ikke tæller).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side-URL-identifikator (ryddet server-side). |
| afterName | string | query | No | Markør: send nextAfterName fra den forrige respons. |
| afterUserId | string | query | No | Markør tie-breaker: send nextAfterUserId fra den forrige respons. Kræves når afterName er sat så navne-ties ikke dropper poster. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Side-URL-identifikator (ryddet server-side).
let afterName = "afterName_example" // String | Markør: send nextAfterName fra den forrige respons. (optional)
let afterUserId = "afterUserId_example" // String | Markør tie-breaker: send nextAfterUserId fra den forrige respons. Kræves når afterName er sat så navne-ties ikke dropper poster. (optional)

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