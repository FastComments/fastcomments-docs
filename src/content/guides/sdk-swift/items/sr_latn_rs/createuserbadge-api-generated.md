## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APICreateUserBadgeResponse.swift)

## Primer

[inline-code-attrs-start title = 'createUserBadge Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek u beta fazi. Za bilo koji problem, prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createUserBadgeParams = CreateUserBadgeParams(userId: "userId_example", badgeId: "badgeId_example", displayedOnComments: false) // CreateUserBadgeParams | 

DefaultAPI.createUserBadge(tenantId: tenantId, createUserBadgeParams: createUserBadgeParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]