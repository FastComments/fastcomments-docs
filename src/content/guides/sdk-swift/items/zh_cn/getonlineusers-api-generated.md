---
当前在线的页面查看者：其 websocket 会话当前已订阅该页面的人。
返回 anonCount + totalCount（房间范围的订阅者，包括我们不列举的匿名查看者）。

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（服务器端已清理）。 |
| afterName | string | query | 否 | 游标：传递上一个响应中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游标平局决胜：传递上一个响应中的 nextAfterUserId。在设置了 afterName 时必需，以避免同名情况下丢失条目。 |

## 响应

返回：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## 示例

[inline-code-attrs-start title = 'getOnlineUsers 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 页面 URL 标识符（服务器端已清理）。
let afterName = "afterName_example" // String | 游标：传递上一次响应中的 nextAfterName。（可选）
let afterUserId = "afterUserId_example" // String | 游标平局决胜：传递上一次响应中的 nextAfterUserId。当设置了 afterName 时为必需，以防同名导致条目丢失。（可选）

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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