## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | Используется для определения, подписана ли текущая страница. |
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

## Ответ

Возвращает: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Последующие образцы кода находятся в бета‑версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Используется для определения, подписана ли текущая страница. (необязательно)
let pageSize = 987 // Int | (необязательно)
let afterId = "afterId_example" // String | (необязательно)
let includeContext = true // Bool | (необязательно)
let afterCreatedAt = 987 // Int64 | (необязательно)
let unreadOnly = true // Bool | (необязательно)
let dmOnly = true // Bool | (необязательно)
let noDm = true // Bool | (необязательно)
let includeTranslations = true // Bool | (необязательно)
let includeTenantNotifications = true // Bool | (необязательно)
let sso = "sso_example" // String | (необязательно)

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