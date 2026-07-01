List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 不透明的分页游标，由先前请求返回的 `nextCursor` 提供。与相同的 `sortBy` 关联。 |
| limit | integer | query | No | 1..200，默认 50 |
| q | string | query | No | 可选的大小写不敏感的标题前缀过滤器。 |
| sortBy | string | query | No | 排序顺序。`updatedAt`（默认，最新优先），`commentCount`（评论数最高优先），或 `title`（字母顺序）。 |
| hasComments | boolean | query | No | 如果为 true，则仅返回至少有一条评论的页面。 |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 不透明的分页游标，由先前请求返回的 `nextCursor` 提供。与相同的 `sortBy` 关联。（可选）
let limit = 987 // Int | 1..200，默认 50（可选）
let q = "q_example" // String | 可选的大小写不敏感的标题前缀过滤器。（可选）
let sortBy = PagesSortBy() // PagesSortBy | 排序顺序。`updatedAt`（默认，最新优先），`commentCount`（评论数最高优先），或 `title`（字母顺序）。（可选）
let hasComments = true // Bool | 如果为 true，则仅返回至少有一条评论的页面。（可选）

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

---