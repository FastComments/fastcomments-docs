## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| afterId | string | query | 아니요 |  |
| afterCreatedAt | integer | query | 아니요 |  |
| unreadOnly | boolean | query | 아니요 |  |
| dmOnly | boolean | query | 아니요 |  |
| noDm | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ResetUserNotifications200Response.swift)

## 예제

[inline-code-attrs-start title = 'resetUserNotifications 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (선택 사항)
let afterCreatedAt = 987 // Int64 |  (선택 사항)
let unreadOnly = true // Bool |  (선택 사항)
let dmOnly = true // Bool |  (선택 사항)
let noDm = true // Bool |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

PublicAPI.resetUserNotifications(tenantId: tenantId, afterId: afterId, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---