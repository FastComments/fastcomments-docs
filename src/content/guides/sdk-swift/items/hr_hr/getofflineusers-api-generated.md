Prošli komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije "Members".
Cursor paginacija na commenterName: server prolazi djelomični {tenantId, urlId, commenterName}
indeks od afterName naprijed preko $gt, bez troška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (obrađen na serverskoj strani). |
| afterName | string | query | No | Kursor: pošaljite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor za razrješavanje neriješenosti: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unos s istim imenom ne bi pao s popisa. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo kakav problem, prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL stranice (obrađen na serverskoj strani).
let afterName = "afterName_example" // String | Kursor: pošaljite nextAfterName iz prethodnog odgovora. (neobavezno)
let afterUserId = "afterUserId_example" // String | Kursor za razrješavanje neriješenosti: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unos s istim imenom ne bi pao s popisa. (neobavezno)

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