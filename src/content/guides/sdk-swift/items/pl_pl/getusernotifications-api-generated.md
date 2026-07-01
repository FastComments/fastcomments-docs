## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | Używane do określenia, czy bieżąca strona jest subskrybowana. |
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

## Odpowiedź

Zwraca: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W razie jakichkolwiek problemów prosimy zgłaszać je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Używane do określenia, czy bieżąca strona jest subskrybowana. (opcjonalnie)
let pageSize = 987 // Int |  (opcjonalnie)
let afterId = "afterId_example" // String |  (opcjonalnie)
let includeContext = true // Bool |  (opcjonalnie)
let afterCreatedAt = 987 // Int64 |  (opcjonalnie)
let unreadOnly = true // Bool |  (opcjonalnie)
let dmOnly = true // Bool |  (opcjonalnie)
let noDm = true // Bool |  (opcjonalnie)
let includeTranslations = true // Bool |  (opcjonalnie)
let includeTenantNotifications = true // Bool |  (opcjonalnie)
let sso = "sso_example" // String |  (opcjonalnie)

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