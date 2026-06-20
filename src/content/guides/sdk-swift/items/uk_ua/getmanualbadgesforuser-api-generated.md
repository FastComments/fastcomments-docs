## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Ні |  |
| commentId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserManualBadgesResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getManualBadgesForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. Для будь-яких проблем, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgesUserId = "badgesUserId_example" // String |  (необов'язково)
let commentId = "commentId_example" // String |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

ModerationAPI.getManualBadgesForUser(badgesUserId: badgesUserId, commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]