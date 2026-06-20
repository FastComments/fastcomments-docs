## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| userId | string | query | いいえ |  |
| trustFactor | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## 例

[inline-code-attrs-start title = 'setTrustFactor の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let userId = "userId_example" // String |  (オプション)
let trustFactor = "trustFactor_example" // String |  (オプション)
let sso = "sso_example" // String |  (オプション)

ModerationAPI.setTrustFactor(userId: userId, trustFactor: trustFactor, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]