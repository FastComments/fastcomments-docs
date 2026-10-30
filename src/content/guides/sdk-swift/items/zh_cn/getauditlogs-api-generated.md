## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | 要读取审计日志的租户。 |
| limit | integer | query | No | 返回多少条日志。最大 10000，默认 1000。 |
| skip | integer | query | No | 要跳过的日志数量，用于分页。 |
| order | string | query | No | 事件时间的排序方向。默认 DESC（最新的在前）。 |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | 仅返回此用户名执行的事件。 |
| ip | string | query | No | 仅返回来自此 IP 地址的事件。 |
| crudType | string | query | No | 仅返回此类型的事件：c、r、u、d 或 login。 |
| resourceName | string | query | No | 仅返回此资源的事件，例如 User 或 Moderator。 |
| targetId | string | query | No | 仅返回受影响对象具有此 ID 的事件。 |
| target | string | query | No | 对受影响对象的标签进行不区分大小写的子字符串匹配，例如用户名或电子邮件。 |
| includeManagedTenants | boolean | query | No | 还返回此租户管理的租户的事件。 |

## 响应

返回：[`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## 示例

[inline-code-attrs-start title = 'getAuditLogs 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 要读取审计日志的租户。
let limit = 987 // Int | 返回多少条日志。最大 10000，默认 1000。（可选）
let skip = 987 // Int | 要跳过的日志数量，用于分页。（可选）
let order = SORT_DIR() // SORTDIR | 事件时间的排序方向。默认 DESC（最新的在前）。（可选）
let after = 987 // Int64 |  （可选）
let before = 987 // Int64 |  （可选）
let username = "username_example" // String | 仅返回此用户名执行的事件。（可选）
let ip = "ip_example" // String | 仅返回来自此 IP 地址的事件。（可选）
let crudType = "crudType_example" // String | 仅返回此类型的事件：c、r、u、d 或 login。（可选）
let resourceName = "resourceName_example" // String | 仅返回此资源的事件，例如 User 或 Moderator。（可选）
let targetId = "targetId_example" // String | 仅返回受影响对象具有此 ID 的事件。（可选）
let target = "target_example" // String | 对受影响对象的标签进行不区分大小写的子字符串匹配，例如用户名或电子邮件。（可选）
let includeManagedTenants = true // Bool | 还返回此租户管理的租户的事件。（可选）

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]