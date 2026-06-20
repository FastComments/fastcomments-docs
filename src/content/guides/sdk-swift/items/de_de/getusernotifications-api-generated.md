---
## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | Wird verwendet, um festzustellen, ob die aktuelle Seite abonniert ist. |
| pageSize | integer | query | No |  |
| afterId | string | query | No |  |
| includeContext | boolean | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| includeTranslations | boolean | query | No |  |
| includeTenantNotifications | boolean | query | No |  |
| sso | string | query | No |  |

## Antwort

Gibt zurück: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getUserNotifications Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie sich bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Wird verwendet, um festzustellen, ob die aktuelle Seite abonniert ist. (optional)
let pageSize = 987 // Int |  (optional)
let afterId = "afterId_example" // String |  (optional)
let includeContext = true // Bool |  (optional)
let afterCreatedAt = 987 // Int64 |  (optional)
let unreadOnly = true // Bool |  (optional)
let dmOnly = true // Bool |  (optional)
let noDm = true // Bool |  (optional)
let includeTranslations = true // Bool |  (optional)
let includeTenantNotifications = true // Bool |  (optional)
let sso = "sso_example" // String |  (optional)

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

---