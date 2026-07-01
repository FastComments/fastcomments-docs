## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |
| skip | number | query | No |  |

## 응답

반환: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventsResponse.swift)

## 예제

[inline-code-attrs-start title = 'getPendingWebhookEvents 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 위 코드는 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (옵션)
let externalId = "externalId_example" // String |  (옵션)
let eventType = "eventType_example" // String |  (옵션)
let type = "type_example" // String |  (옵션)
let domain = "domain_example" // String |  (옵션)
let attemptCountGT = 987 // Double |  (옵션)
let skip = 987 // Double |  (옵션)

DefaultAPI.getPendingWebhookEvents(tenantId: tenantId, options: DefaultAPI.GetPendingWebhookEventsOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]