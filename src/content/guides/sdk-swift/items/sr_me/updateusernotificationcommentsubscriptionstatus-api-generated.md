Омогућите или онемогућите обавештења за одређени коментар.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| notificationId | string | path | Да |  |
| optedInOrOut | string | path | Да |  |
| commentId | string | query | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationCommentSubscriptionStatusResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример updateUserNotificationCommentSubscriptionStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let notificationId = "notificationId_example" // String | 
let optedInOrOut = "optedInOrOut_example" // String | 
let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (опционо)

PublicAPI.updateUserNotificationCommentSubscriptionStatus(tenantId: tenantId, notificationId: notificationId, optedInOrOut: optedInOrOut, commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]