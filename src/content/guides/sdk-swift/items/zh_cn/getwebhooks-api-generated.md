列出为租户配置的 webhook，包括仪表板管理的行和 API 订阅。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| event | string | query | 否 |  |
| domain | string | query | 否 |  |
| source | string | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## 示例

[inline-code-attrs-start title = 'getWebhooks 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  （可选）
let domain = "domain_example" // String |  （可选）
let source = WebhookSource() // WebhookSource |  （可选）
let skip = 987 // Double |  （可选）

DefaultAPI.getWebhooks(tenantId: tenantId, options: DefaultAPI.GetWebhooksOptions(event: event, domain: domain, source: source, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]