## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nein |  |
| badgeId | string | query | Nein |  |
| type | number | query | Nein |  |
| displayedOnComments | boolean | query | Nein |  |
| limit | number | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserBadges200Response.swift)

## Beispiel

[inline-code-attrs-start title = 'getUserBadges Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bitte melden Sie Probleme unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (optional)
let badgeId = "badgeId_example" // String |  (optional)
let type = 987 // Double |  (optional)
let displayedOnComments = true // Bool |  (optional)
let limit = 987 // Double |  (optional)
let skip = 987 // Double |  (optional)

DefaultAPI.getUserBadges(tenantId: tenantId, userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip) { (response, error) in
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