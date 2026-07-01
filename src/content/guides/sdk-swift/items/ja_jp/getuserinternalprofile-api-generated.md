## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|------|------|
| tenantId | string | query | はい |  |
| commentId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

返却: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## 例

[inline-code-attrs-start title = 'getUserInternalProfile の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new から報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // 文字列 | 
let commentId = "commentId_example" // 文字列 | （オプション）
let sso = "sso_example" // 文字列 | （オプション）

ModerationAPI.getUserInternalProfile(tenantId: tenantId, options: ModerationAPI.GetUserInternalProfileOptions(commentId: commentId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]