## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Не | Користи се да се утврди да ли је тренутна страница претплаћена. |
| pageSize | integer | query | Не |  |
| afterId | string | query | Не |  |
| includeContext | boolean | query | Не |  |
| afterCreatedAt | integer | query | Не |  |
| unreadOnly | boolean | query | Не |  |
| dmOnly | boolean | query | Не |  |
| noDm | boolean | query | Не |  |
| includeTranslations | boolean | query | Не |  |
| includeTenantNotifications | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем пријавите на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Користи се да се утврди да ли је тренутна страница претплаћена. (опционо)
let pageSize = 987 // Int |  (опционо)
let afterId = "afterId_example" // String |  (опционо)
let includeContext = true // Bool |  (опционо)
let afterCreatedAt = 987 // Int64 |  (опционо)
let unreadOnly = true // Bool |  (опционо)
let dmOnly = true // Bool |  (опционо)
let noDm = true // Bool |  (опционо)
let includeTranslations = true // Bool |  (опционо)
let includeTenantNotifications = true // Bool |  (опционо)
let sso = "sso_example" // String |  (опционо)

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