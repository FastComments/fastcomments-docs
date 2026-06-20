Trenutno aktivni gledalci strani: osebe, katerih websocket seja je trenutno naročena na stran.
Vrača anonCount + totalCount (naročniki po sobi, vključno z anonimnimi gledalci, ki jih ne navajamo).

## Parametri

| Ime | Tip | Location | Required | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL strani (očiščen na strežniku). |
| afterName | string | query | Ne | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Kazalec za razvezovanje (tiebreaker): posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljeno, da vnosi z enakim imenom ne bodo izpuščeni. |

## Odgovor

Vrača: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za kakršnokoli težavo, prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL strani (očiščen na strežniku).
let afterName = "afterName_example" // String | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. (neobvezno)
let afterUserId = "afterUserId_example" // String | Kazalec za razvezovanje (tiebreaker): posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljeno, da vnosi z enakim imenom ne bodo izpuščeni. (neobvezno)

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