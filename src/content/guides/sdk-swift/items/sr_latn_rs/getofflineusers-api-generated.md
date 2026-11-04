Prošli komentatori na stranici koji trenutno NISU na mreži. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije "Members".
Kursorska paginacija po commenterName: server prolazi kroz parcijalni {tenantId, urlId, commenterName}
indeks od afterName naprijed putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očišćen na serveru). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursorski tiebreaker: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se unosi sa istim imenom ne bi izgubili. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Primjer

[inline-code-attrs-start title = 'getOfflineUsers Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL-a stranice (očišćen na serveru).
let afterName = "afterName_example" // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (optional)
let afterUserId = "afterUserId_example" // String | Kursorski tiebreaker: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se unosi sa istim imenom ne bi izgubili. (optional)

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