## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| pageSize | integer | query | Nee |  |
| afterId | string | query | Nee |  |
| includeContext | boolean | query | Nee |  |
| afterCreatedAt | integer | query | Nee |  |
| unreadOnly | boolean | query | Nee |  |
| dmOnly | boolean | query | Nee |  |
| noDm | boolean | query | Nee |  |
| includeTranslations | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Geeft terug: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserNotifications200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getUserNotifications Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Bij problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let pageSize = 987 // Int |  (optioneel)
let afterId = "afterId_example" // String |  (optioneel)
let includeContext = true // Bool |  (optioneel)
let afterCreatedAt = 987 // Int64 |  (optioneel)
let unreadOnly = true // Bool |  (optioneel)
let dmOnly = true // Bool |  (optioneel)
let noDm = true // Bool |  (optioneel)
let includeTranslations = true // Bool |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

PublicAPI.getUserNotifications(tenantId: tenantId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, sso: sso) { (response, error) in
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