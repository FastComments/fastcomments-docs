Trenutno spletni gledalci strani: ljudje, katerih se seja WebSocket naroči na stran zdaj.  
Vrne anonCount + totalCount (naročniki po celotni sobi, vključno z anonimnimi gledalci, ki jih ne izštejemo).

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL naslova strani (čiščen na strežniku). |
| afterName | string | query | Ne | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Razreševalnik za enake kazalce: posredujte nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da se pri enakih imenih ne izpustijo vnosi. |

## Odgovor

Vrne: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za morebitne težave prosimo prijavite jih preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL naslova strani (čiščen na strežniku).
let afterName = "afterName_example" // String | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. (optional)
let afterUserId = "afterUserId_example" // String | Razreševalnik za enake kazalce: posredujte nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da se pri enakih imenih ne izpustijo vnosi. (optional)

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