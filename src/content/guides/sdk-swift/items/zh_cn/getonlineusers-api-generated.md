当前在线的页面查看者：指其 websocket 会话当前已订阅该页面的用户。  
返回 anonCount + totalCount（全房间订阅者，包括我们不枚举的匿名查看者）。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 页面 URL 标识符（服务器端清理后）。 |
| afterName | string | query | No | 光标：从上一次响应中传递 nextAfterName。 |
| afterUserId | string | query | No | 光标平局解决器：从上一次响应中传递 nextAfterUserId。当设置 afterName 时需要此字段，以防止名称平局导致条目丢失。 |

## 响应

返回：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## 示例

[inline-code-attrs-start title = '获取在线用户 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 页面 URL 标识符（服务器端清理后）。
let afterName = "afterName_example" // String | 光标：从上一次响应中传递 nextAfterName。（可选）
let afterUserId = "afterUserId_example" // String | 光标平局解决器：从上一次响应中传递 nextAfterUserId。当设置 afterName 时需要此字段，以防止名称平局导致条目丢失。（可选）

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
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