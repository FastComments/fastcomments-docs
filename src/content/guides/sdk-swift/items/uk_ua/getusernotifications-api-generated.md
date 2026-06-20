## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| urlId | string | query | Ні | Використовується для визначення, чи підписана поточна сторінка. |
| pageSize | integer | query | Ні |  |
| afterId | string | query | Ні |  |
| includeContext | boolean | query | Ні |  |
| afterCreatedAt | integer | query | Ні |  |
| unreadOnly | boolean | query | Ні |  |
| dmOnly | boolean | query | Ні |  |
| noDm | boolean | query | Ні |  |
| includeTranslations | boolean | query | Ні |  |
| includeTenantNotifications | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. Якщо виникне проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Використовується для визначення, чи підписана поточна сторінка. (необов'язково)
let pageSize = 987 // Int |  (необов'язково)
let afterId = "afterId_example" // String |  (необов'язково)
let includeContext = true // Bool |  (необов'язково)
let afterCreatedAt = 987 // Int64 |  (необов'язково)
let unreadOnly = true // Bool |  (необов'язково)
let dmOnly = true // Bool |  (необов'язково)
let noDm = true // Bool |  (необов'язково)
let includeTranslations = true // Bool |  (необов'язково)
let includeTenantNotifications = true // Bool |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

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