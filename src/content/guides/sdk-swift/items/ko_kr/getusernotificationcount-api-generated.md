## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserNotificationCountResponse.swift)

## 예제

[inline-code-attrs-start title = 'getUserNotificationCount 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let sso = "sso_example" // String |  (선택 사항)

PublicAPI.getUserNotificationCount(tenantId: tenantId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]