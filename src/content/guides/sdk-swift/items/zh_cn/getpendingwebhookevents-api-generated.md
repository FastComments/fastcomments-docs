## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| commentId | string | 查询 | 否 |  |
| externalId | string | 查询 | 否 |  |
| eventType | string | 查询 | 否 |  |
| type | string | 查询 | 否 |  |
| domain | string | 查询 | 否 |  |
| attemptCountGT | number | 查询 | 否 |  |
| skip | number | 查询 | 否 |  |

## 响应

返回: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## 示例

[inline-code-attrs-start title = 'getPendingWebhookEvents 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如遇任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (可选)
let externalId = "externalId_example" // String |  (可选)
let eventType = "eventType_example" // String |  (可选)
let type = "type_example" // String |  (可选)
let domain = "domain_example" // String |  (可选)
let attemptCountGT = 987 // Double |  (可选)
let skip = 987 // Double |  (可选)

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

---