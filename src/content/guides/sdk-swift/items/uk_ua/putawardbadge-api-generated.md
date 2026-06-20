## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Так |  |
| userId | string | query | Ні |  |
| commentId | string | query | Ні |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AwardUserBadgeResponse.swift)

## Приклад

[inline-code-attrs-start title = 'putAwardBadge Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще знаходяться на стадії бета. Якщо виникнуть проблеми, будь ласка, повідомте про це через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (необов'язково)
let commentId = "commentId_example" // String |  (необов'язково)
let broadcastId = "broadcastId_example" // String |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

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