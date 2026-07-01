## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | Wordt gebruikt om te bepalen of de huidige pagina geabonneerd is. |
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

## Response

Retourneert: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Example

[inline-code-attrs-start title = 'getUserNotifications Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor eventuele problemen, rapporteer via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Wordt gebruikt om te bepalen of de huidige pagina geabonneerd is. (optioneel)
let pageSize = 987 // Int |  (optioneel)
let afterId = "afterId_example" // String |  (optioneel)
let includeContext = true // Bool |  (optioneel)
let afterCreatedAt = 987 // Int64 |  (optioneel)
let unreadOnly = true // Bool |  (optioneel)
let dmOnly = true // Bool |  (optioneel)
let noDm = true // Bool |  (optioneel)
let includeTranslations = true // Bool |  (optioneel)
let includeTenantNotifications = true // Bool |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

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