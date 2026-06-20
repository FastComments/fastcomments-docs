Trenutno online gledatelji stranice: osobe čija je websocket sesija pretplaćena na stranicu upravo sada.
Vraća anonCount + totalCount (pretplatnike po sobi, uključujući anonimne gledatelje koje ne nabrajamo).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (čišćen na serverskoj strani). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor za razrješavanje neriješenih slučajeva: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se unosi s istim imenom ne bi izgubili. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Primjer

[inline-code-attrs-start title = 'getOnlineUsers Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL-a stranice (čišćen na serverskoj strani).
let afterName = "afterName_example" // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (neobavezno)
let afterUserId = "afterUserId_example" // String | Kursor za razrješavanje neriješenih slučajeva: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se unosi s istim imenom ne bi izgubili. (neobavezno)

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