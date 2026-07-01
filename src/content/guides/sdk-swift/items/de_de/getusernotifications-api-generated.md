## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|--------------|--------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nein | Wird verwendet, um zu bestimmen, ob die aktuelle Seite abonniert ist. |
| pageSize | integer | query | Nein |  |
| afterId | string | query | Nein |  |
| includeContext | boolean | query | Nein |  |
| afterCreatedAt | integer | query | Nein |  |
| unreadOnly | boolean | query | Nein |  |
| dmOnly | boolean | query | Nein |  |
| noDm | boolean | query | Nein |  |
| includeTranslations | boolean | query | Nein |  |
| includeTenantNotifications | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getUserNotifications Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta-Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Wird verwendet, um zu bestimmen, ob die aktuelle Seite abonniert ist. (optional)
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

PublicAPI.getUserNotifications(tenantId: tenantId, options: PublicAPI.GetUserNotificationsOptions(urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]