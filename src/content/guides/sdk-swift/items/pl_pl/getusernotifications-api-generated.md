## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| urlId | string | query | Nie | Służy do określenia, czy bieżąca strona jest zasubskrybowana. |
| pageSize | integer | query | Nie |  |
| afterId | string | query | Nie |  |
| includeContext | boolean | query | Nie |  |
| afterCreatedAt | integer | query | Nie |  |
| unreadOnly | boolean | query | Nie |  |
| dmOnly | boolean | query | Nie |  |
| noDm | boolean | query | Nie |  |
| includeTranslations | boolean | query | Nie |  |
| includeTenantNotifications | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w fazie beta. W razie problemu zgłoś na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Służy do określenia, czy bieżąca strona jest zasubskrybowana. (opcjonalne)
let pageSize = 987 // Int |  (opcjonalne)
let afterId = "afterId_example" // String |  (opcjonalne)
let includeContext = true // Bool |  (opcjonalne)
let afterCreatedAt = 987 // Int64 |  (opcjonalne)
let unreadOnly = true // Bool |  (opcjonalne)
let dmOnly = true // Bool |  (opcjonalne)
let noDm = true // Bool |  (opcjonalne)
let includeTranslations = true // Bool |  (opcjonalne)
let includeTenantNotifications = true // Bool |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

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