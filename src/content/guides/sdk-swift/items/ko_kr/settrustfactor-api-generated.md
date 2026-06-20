## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | 아니오 |  |
| trustFactor | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## 예제

[inline-code-attrs-start title = 'setTrustFactor 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해 주세요
import FastCommentsSwift

let userId = "userId_example" // String |  (선택 사항)
let trustFactor = "trustFactor_example" // String |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

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