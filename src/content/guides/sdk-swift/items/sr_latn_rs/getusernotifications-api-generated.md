## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne | Koristi se da se odredi da li je trenutna stranica pretplaćena. |
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

Vraća: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek beta. Za bilo koji problem, prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Koristi se da se odredi da li je trenutna stranica pretplaćena. (opciono)
let pageSize = 987 // Int |  (opciono)
let afterId = "afterId_example" // String |  (opciono)
let includeContext = true // Bool |  (opciono)
let afterCreatedAt = 987 // Int64 |  (opciono)
let unreadOnly = true // Bool |  (opciono)
let dmOnly = true // Bool |  (opciono)
let noDm = true // Bool |  (opciono)
let includeTranslations = true // Bool |  (opciono)
let includeTenantNotifications = true // Bool |  (opciono)
let sso = "sso_example" // String |  (opciono)

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