## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| fromCommentId | string | query | 아니오 |  |
| viewed | boolean | query | 아니오 |  |
| type | string | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationsResponse.swift)

## 예시

[inline-code-attrs-start title = 'getNotifications 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 으로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (옵션)
let urlId = "urlId_example" // String |  (옵션)
let fromCommentId = "fromCommentId_example" // String |  (옵션)
let viewed = true // Bool |  (옵션)
let type = "type_example" // String |  (옵션)
let skip = 987 // Double |  (옵션)

DefaultAPI.getNotifications(tenantId: tenantId, options: DefaultAPI.GetNotificationsOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]