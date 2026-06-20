## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | Koristi se za utvrđivanje je li trenutna stranica pretplaćena. |
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

## Odgovor

Vraća: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite ga preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Koristi se za utvrđivanje je li trenutna stranica pretplaćena. (neobavezno)
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

---