## Parametri

| Name | Type | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne | Uporablja se za določitev, ali je trenutna stran naročena. |
| pageSize | integer | query | Ne |  |
| afterId | string | query | Ne |  |
| includeContext | boolean | query | Ne |  |
| afterCreatedAt | integer | query | Ne |  |
| unreadOnly | boolean | query | Ne |  |
| dmOnly | boolean | query | Ne |  |
| noDm | boolean | query | Ne |  |
| includeTranslations | boolean | query | Ne |  |
| includeTenantNotifications | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za kakršnokoli težavo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Uporablja se za določitev, ali je trenutna stran naročena. (izbirno)
let pageSize = 987 // Int |  (izbirno)
let afterId = "afterId_example" // String |  (izbirno)
let includeContext = true // Bool |  (izbirno)
let afterCreatedAt = 987 // Int64 |  (izbirno)
let unreadOnly = true // Bool |  (izbirno)
let dmOnly = true // Bool |  (izbirno)
let noDm = true // Bool |  (izbirno)
let includeTranslations = true // Bool |  (izbirno)
let includeTenantNotifications = true // Bool |  (izbirno)
let sso = "sso_example" // String |  (izbirno)

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