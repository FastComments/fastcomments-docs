## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Да |  |
| userId | string | query | Не |  |
| commentId | string | query | Не |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AwardUserBadgeResponse.swift)

## Примјер

[inline-code-attrs-start title = 'putAwardBadge Примјер'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За сваки проблем, пријавите га путем http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (опционално)
let commentId = "commentId_example" // String |  (опционално)
let broadcastId = "broadcastId_example" // String |  (опционално)
let sso = "sso_example" // String |  (опционално)

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