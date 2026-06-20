## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| banEmail | boolean | query | いいえ |  |
| banEmailDomain | boolean | query | いいえ |  |
| banIP | boolean | query | いいえ |  |
| deleteAllUsersComments | boolean | query | いいえ |  |
| bannedUntil | string | query | いいえ |  |
| isShadowBan | boolean | query | いいえ |  |
| updateId | string | query | いいえ |  |
| banReason | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## 例

[inline-code-attrs-start title = 'postBanUserFromComment の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (オプション)
let banEmailDomain = true // Bool |  (オプション)
let banIP = true // Bool |  (オプション)
let deleteAllUsersComments = true // Bool |  (オプション)
let bannedUntil = "bannedUntil_example" // String |  (オプション)
let isShadowBan = true // Bool |  (オプション)
let updateId = "updateId_example" // String |  (オプション)
let banReason = "banReason_example" // String |  (オプション)
let sso = "sso_example" // String |  (オプション)

ModerationAPI.postBanUserFromComment(commentId: commentId, banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]