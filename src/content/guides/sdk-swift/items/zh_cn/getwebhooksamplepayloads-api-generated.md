Recent comments in exactly the shape webhook deliveries use, for building integrations (for example Zapier
sample data). Every event delivers the same comment object, so `event` only has to be valid.

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| event | string | query | 否 |  |
| limit | number | query | 否 |  |

## 响应

返回: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhookSamplePayloadsResponse.swift)

## 示例

[inline-code-attrs-start title = 'getWebhookSamplePayloads 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  （可选）
let limit = 987 // Double |  （可选）

DefaultAPI.getWebhookSamplePayloads(tenantId: tenantId, options: DefaultAPI.GetWebhookSamplePayloadsOptions(event: event, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]