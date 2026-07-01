## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| badgeId | string | query | Yes |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## レスポンス

返却: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AwardUserBadgeResponse.swift)

## 例

[inline-code-attrs-start title = 'putAwardBadge の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 次のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let badgeId = "badgeId_example" // String |
let userId = "userId_example" // String | (オプション)
let commentId = "commentId_example" // String | (オプション)
let broadcastId = "broadcastId_example" // String | (オプション)
let sso = "sso_example" // String | (オプション)

ModerationAPI.putAwardBadge(tenantId: tenantId, badgeId: badgeId, options: ModerationAPI.PutAwardBadgeOptions(userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]