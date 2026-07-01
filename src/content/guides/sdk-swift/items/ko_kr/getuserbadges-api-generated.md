## Parameters

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| badgeId | string | query | 아니오 |  |
| type | number | query | 아니오 |  |
| displayedOnComments | boolean | query | 아니오 |  |
| limit | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## Response

반환: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## Example

[inline-code-attrs-start title = 'getUserBadges 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (옵션)
let badgeId = "badgeId_example" // String |  (옵션)
let type = 987 // Double |  (옵션)
let displayedOnComments = true // Bool |  (옵션)
let limit = 987 // Double |  (옵션)
let skip = 987 // Double |  (옵션)

DefaultAPI.getUserBadges(tenantId: tenantId, options: DefaultAPI.GetUserBadgesOptions(userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]