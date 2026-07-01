## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 아니오 |  |
| externalId | string | query | 아니오 |  |
| eventType | string | query | 아니오 |  |
| type | string | query | 아니오 |  |
| domain | string | query | 아니오 |  |
| attemptCountGT | number | query | 아니오 |  |

## 응답

반환: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCountResponse.swift)

## 예시

[inline-code-attrs-start title = 'getPendingWebhookEventCount 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | (옵션)
let externalId = "externalId_example" // String | (옵션)
let eventType = "eventType_example" // String | (옵션)
let type = "type_example" // String | (옵션)
let domain = "domain_example" // String | (옵션)
let attemptCountGT = 987 // Double | (옵션)

DefaultAPI.getPendingWebhookEventCount(tenantId: tenantId, options: DefaultAPI.GetPendingWebhookEventCountOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]