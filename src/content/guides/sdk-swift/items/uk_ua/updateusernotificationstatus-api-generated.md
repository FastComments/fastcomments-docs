## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| notificationId | string | path | Так |  |
| newStatus | string | path | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationStatusResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад updateUserNotificationStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. Якщо виникнуть проблеми, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let notificationId = "notificationId_example" // String | 
let newStatus = "newStatus_example" // String | 
let sso = "sso_example" // String |  (необов'язково)

PublicAPI.updateUserNotificationStatus(tenantId: tenantId, notificationId: notificationId, newStatus: newStatus, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]