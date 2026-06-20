Trenutno prisutni gledaoci stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici sobe, uključujući anonimne gledaoce koje ne nabrajamo).

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (pročišćen na serverskoj strani). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor za razrješavanje izjednačenja: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se spriječilo gubljenje stavki kod istih imena. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Primer

[inline-code-attrs-start title = 'getOnlineUsers Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL stranice (pročišćen na serverskoj strani).
let afterName = "afterName_example" // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opcionalno)
let afterUserId = "afterUserId_example" // String | Kursor za razrješavanje izjednačenja: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se spriječilo gubljenje stavki kod istih imena. (opcionalno)

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