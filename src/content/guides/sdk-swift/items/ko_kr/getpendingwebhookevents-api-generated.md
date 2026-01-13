## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 아니요 |  |
| externalId | string | query | 아니요 |  |
| eventType | string | query | 아니요 |  |
| type | string | query | 아니요 |  |
| domain | string | query | 아니요 |  |
| attemptCountGT | number | query | 아니요 |  |
| skip | number | query | 아니요 |  |

## 응답

반환: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## 예제

[inline-code-attrs-start title = 'getPendingWebhookEvents 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 으로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // 문자열 | 
let commentId = "commentId_example" // 문자열 |  (선택 사항)
let externalId = "externalId_example" // 문자열 |  (선택 사항)
let eventType = "eventType_example" // 문자열 |  (선택 사항)
let type = "type_example" // 문자열 |  (선택 사항)
let domain = "domain_example" // 문자열 |  (선택 사항)
let attemptCountGT = 987 // 숫자 |  (선택 사항)
let skip = 987 // 숫자 |  (선택 사항)

DefaultAPI.getPendingWebhookEvents(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]