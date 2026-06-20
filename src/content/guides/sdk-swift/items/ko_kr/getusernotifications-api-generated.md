## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 아니오 | 현재 페이지가 구독되었는지 여부를 결정하는 데 사용됩니다. |
| pageSize | integer | query | 아니오 |  |
| afterId | string | query | 아니오 |  |
| includeContext | boolean | query | 아니오 |  |
| afterCreatedAt | integer | query | 아니오 |  |
| unreadOnly | boolean | query | 아니오 |  |
| dmOnly | boolean | query | 아니오 |  |
| noDm | boolean | query | 아니오 |  |
| includeTranslations | boolean | query | 아니오 |  |
| includeTenantNotifications | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new에 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 현재 페이지가 구독되었는지 여부를 판단하는 데 사용됩니다. (선택 사항)
let pageSize = 987 // Int |  (선택 사항)
let afterId = "afterId_example" // String |  (선택 사항)
let includeContext = true // Bool |  (선택 사항)
let afterCreatedAt = 987 // Int64 |  (선택 사항)
let unreadOnly = true // Bool |  (선택 사항)
let dmOnly = true // Bool |  (선택 사항)
let noDm = true // Bool |  (선택 사항)
let includeTranslations = true // Bool |  (선택 사항)
let includeTenantNotifications = true // Bool |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

PublicAPI.getUserNotifications(tenantId: tenantId, urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]