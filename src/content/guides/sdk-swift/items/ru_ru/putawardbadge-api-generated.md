## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Yes |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Ответ

Возвращает: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AwardUserBadgeResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример putAwardBadge'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бета-версии. Если возникнут проблемы, пожалуйста, сообщите на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (необязательно)
let commentId = "commentId_example" // String |  (необязательно)
let broadcastId = "broadcastId_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.putAwardBadge(badgeId: badgeId, userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso) { (response, error) in
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