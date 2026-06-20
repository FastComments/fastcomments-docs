---
## パラメータ

| Name | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| approved | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentApprovedResponse.swift)

## 例

[inline-code-attrs-start title = 'postSetCommentApprovalStatus の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let approved = true // Bool |  (オプション)
let sso = "sso_example" // String |  (オプション)

ModerationAPI.postSetCommentApprovalStatus(commentId: commentId, approved: approved, sso: sso) { (response, error) in
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