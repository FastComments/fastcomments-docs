## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nej | Bruges til at afgøre, om den aktuelle side er abonneret på. |
| pageSize | integer | query | Nej |  |
| afterId | string | query | Nej |  |
| includeContext | boolean | query | Nej |  |
| afterCreatedAt | integer | query | Nej |  |
| unreadOnly | boolean | query | Nej |  |
| dmOnly | boolean | query | Nej |  |
| noDm | boolean | query | Nej |  |
| includeTranslations | boolean | query | Nej |  |
| includeTenantNotifications | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getUserNotifications Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Bruges til at afgøre, om den aktuelle side er abonneret på. (valgfri)
let pageSize = 987 // Int |  (valgfri)
let afterId = "afterId_example" // String |  (valgfri)
let includeContext = true // Bool |  (valgfri)
let afterCreatedAt = 987 // Int64 |  (valgfri)
let unreadOnly = true // Bool |  (valgfri)
let dmOnly = true // Bool |  (valgfri)
let noDm = true // Bool |  (valgfri)
let includeTranslations = true // Bool |  (valgfri)
let includeTenantNotifications = true // Bool |  (valgfri)
let sso = "sso_example" // String |  (valgfri)

PublicAPI.getUserNotifications(tenantId: tenantId, urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]