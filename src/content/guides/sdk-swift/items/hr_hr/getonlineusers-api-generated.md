Trenutno online gledatelji stranice: osobe čija je WebSocket sesija pretplaćena na stranicu upravo sada.  
Vraća anonCount + totalCount (pretplatnici u cijeloj sobi, uključujući anonimne gledatelje koje ne izlistavamo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL‑a stranice (čišćen na strani poslužitelja). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor razrješenje neriješenih veza: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je postavljen afterName kako bi se izbjeglo izostavljanje unosa zbog podudaranja imena. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Primjer getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL‑a stranice (čišćen na strani poslužitelja).
let afterName = "afterName_example" // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (optional)
let afterUserId = "afterUserId_example" // String | Kursor razrješenje neriješenih veza: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je postavljen afterName kako bi se izbjeglo izostavljanje unosa zbog podudaranja imena. (optional)

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