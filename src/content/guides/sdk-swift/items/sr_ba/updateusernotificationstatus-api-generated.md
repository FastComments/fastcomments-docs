## Параметри

| Ime | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| notificationId | string | path | Да |  |
| newStatus | string | path | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationStatusResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример updateUserNotificationStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За било који проблем, молимо пријавите преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let notificationId = "notificationId_example" // String | 
let newStatus = "newStatus_example" // String | 
let sso = "sso_example" // String |  (опционо)

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

---