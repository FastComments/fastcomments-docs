## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| badgesUserId | string | query | Нет |  |
| commentId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Returns: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserManualBadgesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getManualBadgesForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода находятся в бета-версии. При любых проблемах, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let badgesUserId = "badgesUserId_example" // String |  (необязательно)
let commentId = "commentId_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getManualBadgesForUser(tenantId: tenantId, options: ModerationAPI.GetManualBadgesForUserOptions(badgesUserId: badgesUserId, commentId: commentId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]