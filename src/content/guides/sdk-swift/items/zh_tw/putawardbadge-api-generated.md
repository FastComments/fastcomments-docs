## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| badgeId | string | query | 是 |  |
| userId | string | query | 否 |  |
| commentId | string | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AwardUserBadgeResponse.swift)

## 範例

[inline-code-attrs-start title = 'putAwardBadge 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍為測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (選用)
let commentId = "commentId_example" // String |  (選用)
let broadcastId = "broadcastId_example" // String |  (選用)
let sso = "sso_example" // String |  (選用)

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