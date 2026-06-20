## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Не |  |
| afterCreatedAt | integer | query | Не |  |
| unreadOnly | boolean | query | Не |  |
| dmOnly | boolean | query | Не |  |
| noDm | boolean | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ResetUserNotificationsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за resetUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета. Ако имате проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (незадължително)
let afterCreatedAt = 987 // Int64 |  (незадължително)
let unreadOnly = true // Bool |  (незадължително)
let dmOnly = true // Bool |  (незадължително)
let noDm = true // Bool |  (незадължително)
let sso = "sso_example" // String |  (незадължително)

PublicAPI.resetUserNotifications(tenantId: tenantId, afterId: afterId, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]