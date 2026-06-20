## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## 例

[inline-code-attrs-start title = 'getUserInternalProfile の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let commentId = "commentId_example" // String |  (任意)
let sso = "sso_example" // String |  (任意)

ModerationAPI.getUserInternalProfile(commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]