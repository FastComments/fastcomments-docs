Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (čišćen na serveru). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor razdvajanje: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako imena koja se podudaraju ne bi isključila stavke. |

## Response

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL-a stranice (čišćen na serveru).
let afterName = "afterName_example" // String | Kursor: prosledite nextAfterName iz prethodnog odgovora. (opciono)
let afterUserId = "afterUserId_example" // String | Kursor razdvajanje: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako imena koja se podudaraju ne bi isključila stavke. (opciono)

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