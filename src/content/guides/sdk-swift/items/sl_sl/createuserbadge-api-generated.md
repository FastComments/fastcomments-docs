## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | poizvedba | Da |  |

## Odgovor

Vrača: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APICreateUserBadgeResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer createUserBadge'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za morebitne težave jih prijavite preko http://github.com/OpenAPITools/openapi-generator/issues/new
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

---