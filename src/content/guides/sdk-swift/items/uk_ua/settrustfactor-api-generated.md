## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Ні |  |
| trustFactor | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад setTrustFactor'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. Якщо виникне проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (необов'язково)
let trustFactor = "trustFactor_example" // String |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

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