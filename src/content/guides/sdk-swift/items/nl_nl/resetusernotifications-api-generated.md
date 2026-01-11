---
## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| afterId | string | query | Nee |  |
| afterCreatedAt | integer | query | Nee |  |
| unreadOnly | boolean | query | Nee |  |
| dmOnly | boolean | query | Nee |  |
| noDm | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ResetUserNotifications200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'resetUserNotifications Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (optioneel)
let afterCreatedAt = 987 // Int64 |  (optioneel)
let unreadOnly = true // Bool |  (optioneel)
let dmOnly = true // Bool |  (optioneel)
let noDm = true // Bool |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

PublicAPI.resetUserNotifications(tenantId: tenantId, afterId: afterId, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, sso: sso) { (response, error) in
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