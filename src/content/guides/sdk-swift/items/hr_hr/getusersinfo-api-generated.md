Masovni podaci o korisnicima za tenant. Za zadane userIds, vraća prikazne informacije iz User / SSOUser.
Koristi se u widgetu za komentare za obogaćivanje korisnika koji su se upravo pojavili putem presence event.
Nema konteksta stranice: privatnost se provodi jednako (privatni profili su maskirani).

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | userIds odvojeni zarezom. |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## Primjer

[inline-code-attrs-start title = 'getUsersInfo Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | userIds odvojeni zarezom.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
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