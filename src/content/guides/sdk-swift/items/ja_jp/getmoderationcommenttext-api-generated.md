## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentTextResponse.swift)

## 例

[inline-code-attrs-start title = 'getModerationCommentText の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下記のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new から報告してください
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (オプション)

ModerationAPI.getModerationCommentText(commentId: commentId, sso: sso) { (response, error) in
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