Past komentatorji na strani, ki NI trenutno online. Razvrščeni po displayName.  
Uporabi to po izčrpavanju `/users/online`, da prikažeš razdelek “Člani”.  
Cursor paginacija po commenterName: strežnik hodi po delnem `{tenantId, urlId, commenterName}` indeks od afterName naprej prek $gt, brez stroška $skip.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL strani (očiščen na strežniku). |
| afterName | string | query | Ne | Cursor: posreduj nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Cursor tiebreaker: posreduj nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da se imenski neodločenost ne izpusti vnosi. |

## Odgovor

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za kakršnekoli težave, prosimo, prijavite jih preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL strani (očiščen na strežniku).
let afterName = "afterName_example" // String | Cursor: posreduj nextAfterName iz prejšnjega odgovora. (neobvezno)
let afterUserId = "afterUserId_example" // String | Cursor tiebreaker: posreduj nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da se imenski neodločenost ne izpusti vnosi. (neobvezno)

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

---