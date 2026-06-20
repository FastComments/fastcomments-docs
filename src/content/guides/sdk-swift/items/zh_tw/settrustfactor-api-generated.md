## 參數

| 名稱 | 型別 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| trustFactor | string | query | No |  |
| sso | string | query | No |  |

## 回應

回傳: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## 範例

[inline-code-attrs-start title = 'setTrustFactor 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。如遇任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let userId = "userId_example" // String |  (可選)
let trustFactor = "trustFactor_example" // String |  (可選)
let sso = "sso_example" // String |  (可選)

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