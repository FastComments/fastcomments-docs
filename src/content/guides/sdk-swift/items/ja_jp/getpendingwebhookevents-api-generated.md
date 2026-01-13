## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | query | いいえ |  |
| externalId | string | query | いいえ |  |
| eventType | string | query | いいえ |  |
| type | string | query | いいえ |  |
| domain | string | query | いいえ |  |
| attemptCountGT | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## 例

[inline-code-attrs-start title = 'getPendingWebhookEvents の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (省略可能)
let externalId = "externalId_example" // String |  (省略可能)
let eventType = "eventType_example" // String |  (省略可能)
let type = "type_example" // String |  (省略可能)
let domain = "domain_example" // String |  (省略可能)
let attemptCountGT = 987 // Double |  (省略可能)
let skip = 987 // Double |  (省略可能)

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