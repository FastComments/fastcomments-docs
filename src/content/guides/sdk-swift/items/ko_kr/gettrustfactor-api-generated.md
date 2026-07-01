## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserTrustFactorResponse.swift)

## 예시

[inline-code-attrs-start title = 'getTrustFactor 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 이 아래 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고하십시오
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (옵션)
let sso = "sso_example" // String |  (옵션)

ModerationAPI.getTrustFactor(tenantId: tenantId, options: ModerationAPI.GetTrustFactorOptions(userId: userId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]