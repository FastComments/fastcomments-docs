Trenutno online posetioci stranice: osobe čija je websocket sesija pretplaćena na stranicu upravo sada.  
Vraća anonCount + totalCount (pretplatnici u celokupnoj sobi, uključujući anonimne posetioce koje ne izlistavamo).

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (čistimo na serveru). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor razrešavač podjednakosti: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako se ne bi izostavile stavke zbog podjednakosti imena. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL-a stranice (čistimo na serveru).
let afterName = "afterName_example" // String | Kursor: prosledite nextAfterName iz prethodnog odgovora. (opciono)
let afterUserId = "afterUserId_example" // String | Kursor razrešavač podjednakosti: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako se ne bi izostavile stavke zbog podjednakosti imena. (opciono)

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