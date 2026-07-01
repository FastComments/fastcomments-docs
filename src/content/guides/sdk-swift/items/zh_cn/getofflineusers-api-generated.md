Past commenters on the page who are NOT currently online. Sorted by displayName.  
页面上过去的评论者（当前不在线）。按 displayName 排序。

Use this after exhausting /users/online to render a "Members" section.  
在用尽 /users/online 之后使用，以呈现 “Members” 部分。

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
在 commenterName 上使用游标分页：服务器从 afterName 向前遍历部分 {tenantId, urlId, commenterName} 索引，使用 $gt，没有 $skip 成本。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 页面 URL 标识符（服务器端已清理）。 |
| afterName | string | query | No | 游标：从上一次响应中传递 nextAfterName。（可选） |
| afterUserId | string | query | No | 游标分割键：从上一次响应中传递 nextAfterUserId。当设置 afterName 时需要，以防名称相同导致条目被忽略。（可选） |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 页面 URL 标识符（服务器端已清理）。
let afterName = "afterName_example" // String | 游标：从上一次响应中传递 nextAfterName。（可选）
let afterUserId = "afterUserId_example" // String | 游标分割键：从上一次响应中传递 nextAfterUserId。当设置 afterName 时需要，以防名称相同导致条目被忽略。（可选）

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]