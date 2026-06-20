Prejšnji komentatorji na strani, ki trenutno niso online. Razvrščeni po displayName.
Uporabite to po izčrpanju /users/online, da prikažete odsek "Člani".
Straničenje s kazalom na commenterName: strežnik prehaja delni {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL strani (počiščen na strežniku). |
| afterName | string | query | Ne | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Kazalnik za razrešitev izenačitve: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljeno, da se ob enakih imenih vnosi ne izgubijo. |

## Odgovor

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Primer

[inline-code-attrs-start title = 'getOfflineUsers Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Če naletite na težave, jih prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL strani (počiščen na strežniku).
let afterName = "afterName_example" // String | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. (izbirno)
let afterUserId = "afterUserId_example" // String | Kazalnik za razrešitev izenačitve: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljeno, da se ob enakih imenih vnosi ne izgubijo. (izbirno)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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