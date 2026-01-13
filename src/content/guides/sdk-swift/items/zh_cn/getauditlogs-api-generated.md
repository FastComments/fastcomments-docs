## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| limit | number | query | 否 |  |
| skip | number | query | 否 |  |
| order | string | query | 否 |  |
| after | number | query | 否 |  |
| before | number | query | 否 |  |

## 响应

返回：[`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogs200Response.swift)

## 示例

[inline-code-attrs-start title = 'getAuditLogs 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。若有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (可选)
let skip = 987 // Double |  (可选)
let order = SORT_DIR() // SORTDIR |  (可选)
let after = 987 // Double |  (可选)
let before = 987 // Double |  (可选)

DefaultAPI.getAuditLogs(tenantId: tenantId, limit: limit, skip: skip, order: order, after: after, before: before) { (response, error) in
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