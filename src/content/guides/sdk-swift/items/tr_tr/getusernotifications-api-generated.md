## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| urlId | string | query | Hayır | Geçerli sayfanın abone olup olmadığını belirlemek için kullanılır. |
| pageSize | integer | query | Hayır |  |
| afterId | string | query | Hayır |  |
| includeContext | boolean | query | Hayır |  |
| afterCreatedAt | integer | query | Hayır |  |
| unreadOnly | boolean | query | Hayır |  |
| dmOnly | boolean | query | Hayır |  |
| noDm | boolean | query | Hayır |  |
| includeTranslations | boolean | query | Hayır |  |
| includeTenantNotifications | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Response

Döndürür: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getUserNotifications Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta sürümündedir. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresine bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Geçerli sayfanın abone olup olmadığını belirlemek için kullanılır. (isteğe bağlı)
let pageSize = 987 // Int | (isteğe bağlı)
let afterId = "afterId_example" // String | (isteğe bağlı)
let includeContext = true // Bool | (isteğe bağlı)
let afterCreatedAt = 987 // Int64 | (isteğe bağlı)
let unreadOnly = true // Bool | (isteğe bağlı)
let dmOnly = true // Bool | (isteğe bağlı)
let noDm = true // Bool | (isteğe bağlı)
let includeTranslations = true // Bool | (isteğe bağlı)
let includeTenantNotifications = true // Bool | (isteğe bağlı)
let sso = "sso_example" // String | (isteğe bağlı)

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