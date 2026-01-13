## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| pageSize | integer | query | Nie |  |
| afterId | string | query | Nie |  |
| includeContext | boolean | query | Nie |  |
| afterCreatedAt | integer | query | Nie |  |
| unreadOnly | boolean | query | Nie |  |
| dmOnly | boolean | query | Nie |  |
| noDm | boolean | query | Nie |  |
| includeTranslations | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserNotifications200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let pageSize = 987 // Int |  (opcjonalne)
let afterId = "afterId_example" // String |  (opcjonalne)
let includeContext = true // Bool |  (opcjonalne)
let afterCreatedAt = 987 // Int64 |  (opcjonalne)
let unreadOnly = true // Bool |  (opcjonalne)
let dmOnly = true // Bool |  (opcjonalne)
let noDm = true // Bool |  (opcjonalne)
let includeTranslations = true // Bool |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

PublicAPI.getUserNotifications(tenantId: tenantId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, sso: sso) { (response, error) in
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