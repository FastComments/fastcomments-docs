## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | query | Tak |  |
| userId | string | query | Nie |  |
| badgeId | string | query | Nie |  |
| type | number | query | Nie |  |
| displayedOnComments | boolean | query | Nie |  |
| limit | number | query | Nie |  |
| skip | number | query | Nie |  |

## Response

Zwraca: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W razie jakichkolwiek problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opcjonalny)
let badgeId = "badgeId_example" // String |  (opcjonalny)
let type = 987 // Double |  (opcjonalny)
let displayedOnComments = true // Bool |  (opcjonalny)
let limit = 987 // Double |  (opcjonalny)
let skip = 987 // Double |  (opcjonalny)

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