## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| includeEmail | boolean | query | いいえ |  |
| includeIP | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICommentResponse.swift)

## 例

[inline-code-attrs-start title = 'getModerationComment の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeEmail = true // Bool |  (任意)
let includeIP = true // Bool |  (任意)
let sso = "sso_example" // String |  (任意)

ModerationAPI.getModerationComment(commentId: commentId, includeEmail: includeEmail, includeIP: includeIP, sso: sso) { (response, error) in
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