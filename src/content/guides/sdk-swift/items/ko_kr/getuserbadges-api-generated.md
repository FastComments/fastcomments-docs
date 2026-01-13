---
## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니요 |  |
| badgeId | string | query | 아니요 |  |
| type | number | query | 아니요 |  |
| displayedOnComments | boolean | query | 아니요 |  |
| limit | number | query | 아니요 |  |
| skip | number | query | 아니요 |  |

## 응답

반환: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserBadges200Response.swift)

## 예제

[inline-code-attrs-start title = 'getUserBadges 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (선택 사항)
let badgeId = "badgeId_example" // String |  (선택 사항)
let type = 987 // Double |  (선택 사항)
let displayedOnComments = true // Bool |  (선택 사항)
let limit = 987 // Double |  (선택 사항)
let skip = 987 // Double |  (선택 사항)

DefaultAPI.getUserBadges(tenantId: tenantId, userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip) { (response, error) in
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