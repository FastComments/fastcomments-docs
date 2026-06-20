Trenutno-online posetioci stranice: osobe čija je websocket sesija pretplaćena na stranicu upravo sada.
Vraća anonCount + totalCount (pretplatnici sobe, uključujući anonimne posetioce koje ne navodimo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (obrađen na serverskoj strani). |
| afterName | string | query | No | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor za razrešavanje izjednačenja: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen, kako bi se pri izjednačenju imena stavke ne bi izgubile. |

## Response

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, prijavite ga preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL-a stranice (obrađen na serverskoj strani).
let afterName = "afterName_example" // String | Kursor: prosledite nextAfterName iz prethodnog odgovora. (neobavezno)
let afterUserId = "afterUserId_example" // String | Kursor za razrešavanje izjednačenja: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen, kako bi se pri izjednačenju imena stavke ne bi izgubile. (neobavezno)

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