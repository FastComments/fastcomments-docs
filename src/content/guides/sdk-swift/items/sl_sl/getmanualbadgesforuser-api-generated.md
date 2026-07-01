## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| badgesUserId | string | query | Ne |  |
| commentId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserManualBadgesResponse.swift)

## Primer

[inline-code-attrs-start title = 'getManualBadgesForUser Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za morebitne težave prosimo sporočite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let badgesUserId = "badgesUserId_example" // String |  (neobvezno)
let commentId = "commentId_example" // String |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)

ModerationAPI.getManualBadgesForUser(tenantId: tenantId, options: ModerationAPI.GetManualBadgesForUserOptions(badgesUserId: badgesUserId, commentId: commentId, sso: sso)) { (response, error) in
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