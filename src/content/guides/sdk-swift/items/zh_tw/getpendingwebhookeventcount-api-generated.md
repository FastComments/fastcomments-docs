## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 否 |  |
| externalId | string | query | 否 |  |
| eventType | string | query | 否 |  |
| type | string | query | 否 |  |
| domain | string | query | 否 |  |
| attemptCountGT | number | query | 否 |  |

## 回應

回傳: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCount200Response.swift)

## 範例

[inline-code-attrs-start title = 'getPendingWebhookEventCount 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  （選用）
let externalId = "externalId_example" // String |  （選用）
let eventType = "eventType_example" // String |  （選用）
let type = "type_example" // String |  （選用）
let domain = "domain_example" // String |  （選用）
let attemptCountGT = 987 // Double |  （選用）

DefaultAPI.getPendingWebhookEventCount(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]