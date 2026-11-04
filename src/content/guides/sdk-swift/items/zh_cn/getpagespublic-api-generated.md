列出租户的页面。由 FChat 桌面客户端用于填充其房间列表。
要求每个页面的已解析自定义配置中 `enableFChat` 为 true。
需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 上一次请求返回的 `nextCursor` 不透明分页游标。与相同的 `sortBy` 绑定。 |
| limit | integer | query | 否 | 1..200，默认 50 |
| q | string | query | 否 | 可选的不区分大小写的标题前缀过滤器。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`（默认，最新优先）、`commentCount`（评论最多优先）或 `title`（按字母顺序）。 |
| hasComments | boolean | query | 否 | 如果为 true，则仅返回至少有一条评论的页面。 |

## 响应

返回：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 上一次请求返回的 `nextCursor` 不透明分页游标。与相同的 `sortBy` 绑定。 (optional)
let limit = 987 // Int | 1..200，默认 50 (optional)
let q = "q_example" // String | 可选的不区分大小写的标题前缀过滤器。 (optional)
let sortBy = PagesSortBy() // PagesSortBy | 排序方式。`updatedAt`（默认，最新优先）、`commentCount`（评论最多优先）或 `title`（按字母顺序）。 (optional)
let hasComments = true // Bool | 如果为 true，则仅返回至少有一条评论的页面。 (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]