## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| commentId | string | 查詢 | 否 |  |
| externalId | string | 查詢 | 否 |  |
| eventType | string | 查詢 | 否 |  |
| type | string | 查詢 | 否 |  |
| domain | string | 查詢 | 否 |  |
| attemptCountGT | number | 查詢 | 否 |  |
| skip | number | 查詢 | 否 |  |

## 回應

回傳: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## 範例

[inline-code-attrs-start title = 'getPendingWebhookEvents 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (可選)
let externalId = "externalId_example" // String |  (可選)
let eventType = "eventType_example" // String |  (可選)
let type = "type_example" // String |  (可選)
let domain = "domain_example" // String |  (可選)
let attemptCountGT = 987 // Double |  (可選)
let skip = 987 // Double |  (可選)

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