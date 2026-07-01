Trenutno‑online gledatelji stranice: ljudi čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnike u cijeloj sobi, uključujući anonimne gledatelje koje ne navodimo).

## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL‑a stranice (čišćen na serveru). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opcionalno) |
| afterUserId | string | query | Ne | Kursor razrješavač: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se izbjeglo izostavljanje unosa pri podudaranju imena. |

## Response

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Primjer getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL‑a stranice (čišćen na serveru).
let afterName = "afterName_example" // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opcionalno)
let afterUserId = "afterUserId_example" // String | Kursor razrješavač: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se izbjeglo izostavljanje unosa pri podudaranju imena. (opcionalno)

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