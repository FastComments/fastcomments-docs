## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Не | Използва се за определяне дали текущата страница е абонирана. |
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

## Отговор

Връща: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери за код все още са бета. Ако срещнете проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Използва се за определяне дали текущата страница е абонирана. (незадължително)
let pageSize = 987 // Int |  (незадължително)
let afterId = "afterId_example" // String |  (незадължително)
let includeContext = true // Bool |  (незадължително)
let afterCreatedAt = 987 // Int64 |  (незадължително)
let unreadOnly = true // Bool |  (незадължително)
let dmOnly = true // Bool |  (незадължително)
let noDm = true // Bool |  (незадължително)
let includeTranslations = true // Bool |  (незадължително)
let includeTenantNotifications = true // Bool |  (незадължително)
let sso = "sso_example" // String |  (незадължително)

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