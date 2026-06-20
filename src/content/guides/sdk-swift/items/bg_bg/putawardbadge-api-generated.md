## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Да |  |
| userId | string | query | Не |  |
| commentId | string | query | Не |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AwardUserBadgeResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за putAwardBadge'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни кодове все още са в бета. При проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (незадължително)
let commentId = "commentId_example" // String |  (незадължително)
let broadcastId = "broadcastId_example" // String |  (незадължително)
let sso = "sso_example" // String |  (незадължително)

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