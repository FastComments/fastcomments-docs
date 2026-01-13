## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| pageSize | integer | query | 아니요 |  |
| afterId | string | query | 아니요 |  |
| includeContext | boolean | query | 아니요 |  |
| afterCreatedAt | integer | query | 아니요 |  |
| unreadOnly | boolean | query | 아니요 |  |
| dmOnly | boolean | query | 아니요 |  |
| noDm | boolean | query | 아니요 |  |
| includeTranslations | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserNotifications200Response.swift)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 통해 알려주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let pageSize = 987 // Int |  (선택 사항)
let afterId = "afterId_example" // String |  (선택 사항)
let includeContext = true // Bool |  (선택 사항)
let afterCreatedAt = 987 // Int64 |  (선택 사항)
let unreadOnly = true // Bool |  (선택 사항)
let dmOnly = true // Bool |  (선택 사항)
let noDm = true // Bool |  (선택 사항)
let includeTranslations = true // Bool |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

PublicAPI.getUserNotifications(tenantId: tenantId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]