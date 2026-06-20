---
列出租户的页面。由 FChat 桌面客户端用于填充其房间列表。
要求在每个页面解析后的自定义配置中 `enableFChat` 为 true。
需要 SSO 的页面会根据请求用户的组访问权限进行筛选。

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 不透明分页游标，由先前请求返回的 `nextCursor` 提供。与相同的 `sortBy` 绑定。 |
| limit | integer | query | No | 1..200，默认 50 |
| q | string | query | No | 可选的不区分大小写的标题前缀过滤器。 |
| sortBy | string | query | No | 排序方式。`updatedAt`（默认，按最新在前），`commentCount`（评论最多的在前），或 `title`（按字母顺序）。 |
| hasComments | boolean | query | No | 如果为 true，则只返回至少有一条评论的页面。 |

## 响应

返回：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 不透明分页游标，由先前请求返回的 `nextCursor` 提供。与相同的 `sortBy` 绑定。 (optional)
let limit = 987 // Int | 1..200，默认 50 (optional)
let q = "q_example" // String | 可选的不区分大小写的标题前缀过滤器。 (optional)
let sortBy = PagesSortBy() // PagesSortBy | 排序方式。`updatedAt`（默认，按最新在前），`commentCount`（评论最多的在前），或 `title`（按字母顺序）。 (optional)
let hasComments = true // Bool | 如果为 true，则只返回至少有一条评论的页面。 (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
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