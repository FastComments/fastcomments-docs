Prethodni komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online da biste prikazali sekciju „Članovi“.
Paginacija kursorom po commenterName: server pretražuje parcijalni indeks {tenantId, urlId, commenterName}
indeks od afterName unapred putem $gt, bez $skip troška.

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL stranice (obrađen na serverskoj strani). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Tiebreaker kursora: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako duplikati imena ne bi bili izostavljeni. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL stranice (obrađen na serverskoj strani).
let afterName = "afterName_example" // String | Kursor: prosledite nextAfterName iz prethodnog odgovora. (opciono)
let afterUserId = "afterUserId_example" // String | Tiebreaker kursora: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako duplikati imena ne bi bili izostavljeni. (opciono)

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