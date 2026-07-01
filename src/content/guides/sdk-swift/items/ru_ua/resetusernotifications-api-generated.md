## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|------------|----------|
| tenantId | string | query | Да |  |
| afterId | string | query | Нет |  |
| afterCreatedAt | integer | query | Нет |  |
| unreadOnly | boolean | query | Нет |  |
| dmOnly | boolean | query | Нет |  |
| noDm | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ResetUserNotificationsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример resetUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё бета. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (необязательно)
let afterCreatedAt = 987 // Int64 |  (необязательно)
let unreadOnly = true // Bool |  (необязательно)
let dmOnly = true // Bool |  (необязательно)
let noDm = true // Bool |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

PublicAPI.resetUserNotifications(tenantId: tenantId, options: PublicAPI.ResetUserNotificationsOptions(afterId: afterId, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]