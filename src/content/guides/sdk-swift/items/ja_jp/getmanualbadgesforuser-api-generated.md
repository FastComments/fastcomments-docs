## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | いいえ |  |
| commentId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserManualBadgesResponse.swift)

## 例

[inline-code-attrs-start title = 'getManualBadgesForUser の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new から報告してください
import FastCommentsSwift

let badgesUserId = "badgesUserId_example" // String |  (オプション)
let commentId = "commentId_example" // String |  (オプション)
let sso = "sso_example" // String |  (オプション)

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