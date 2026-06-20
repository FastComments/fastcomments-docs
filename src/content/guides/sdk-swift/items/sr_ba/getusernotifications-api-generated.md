## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne | Koristi se za utvrđivanje da li je trenutna stranica pretplaćena. |
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

## Primjer

[inline-code-attrs-start title = 'getUserNotifications Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo kakav problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Koristi se za utvrđivanje da li je trenutna stranica pretplaćena. (neobavezno)
let pageSize = 987 // Int |  (neobavezno)
let afterId = "afterId_example" // String |  (neobavezno)
let includeContext = true // Bool |  (neobavezno)
let afterCreatedAt = 987 // Int64 |  (neobavezno)
let unreadOnly = true // Bool |  (neobavezno)
let dmOnly = true // Bool |  (neobavezno)
let noDm = true // Bool |  (neobavezno)
let includeTranslations = true // Bool |  (neobavezno)
let includeTenantNotifications = true // Bool |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)

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