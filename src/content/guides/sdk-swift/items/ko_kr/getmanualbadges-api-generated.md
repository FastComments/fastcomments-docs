## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | 아니요 |  |

## 응답

반환: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantManualBadgesResponse.swift)

## 예제

[inline-code-attrs-start title = 'getManualBadges 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해 주세요
import FastCommentsSwift

let sso = "sso_example" // 문자열 |  (선택 사항)

ModerationAPI.getManualBadges(sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]