## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | poizvedba | Da |  |
| userId | string | poizvedba | Ne |  |
| badgeId | string | poizvedba | Ne |  |
| type | number | poizvedba | Ne |  |
| displayedOnComments | boolean | poizvedba | Ne |  |
| limit | number | poizvedba | Ne |  |
| skip | number | poizvedba | Ne |  |

## Odgovor

Vrne: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta različici. Za morebitne težave jih prosimo sporočite prek http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (neobvezno)
let badgeId = "badgeId_example" // String |  (neobvezno)
let type = 987 // Double |  (neobvezno)
let displayedOnComments = true // Bool |  (neobvezno)
let limit = 987 // Double |  (neobvezno)
let skip = 987 // Double |  (neobvezno)

DefaultAPI.getUserBadges(tenantId: tenantId, options: DefaultAPI.GetUserBadgesOptions(userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip)) { (response, error) in
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