## パラメータ

| Name | Type | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| includeByUserIdAndEmail | boolean | query | いいえ |  |
| includeByIP | boolean | query | いいえ |  |
| includeByEmailDomain | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

返却: [`PreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PreBanSummary.swift)

## 例

[inline-code-attrs-start title = 'getPreBanSummary の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeByUserIdAndEmail = true // Bool |  (オプション)
let includeByIP = true // Bool |  (オプション)
let includeByEmailDomain = true // Bool |  (オプション)
let sso = "sso_example" // String |  (オプション)

ModerationAPI.getPreBanSummary(commentId: commentId, includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]