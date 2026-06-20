## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| notificationId | string | path | Да |  |
| newStatus | string | path | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationStatusResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример updateUserNotificationStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие пример(ы) кода всё ещё находятся в бета-версии. Если возникнут проблемы, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let notificationId = "notificationId_example" // String | 
let newStatus = "newStatus_example" // String | 
let sso = "sso_example" // String |  (необязательно)

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