页面上已发表评论但当前不在线的用户。按 displayName 排序。
在用尽 /users/online 后使用此方法以呈现 "成员" 部分。
基于 commenterName 的游标分页：服务器遍历部分索引 {tenantId, urlId, commenterName}
从 afterName 开始向前使用 $gt 进行索引，无 $skip 开销。

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（由服务器端清理）。 |
| afterName | string | query | 否 | 游标：传入上一响应的 nextAfterName。 |
| afterUserId | string | query | 否 | 游标决胜符：传入上一响应的 nextAfterUserId。当设置了 afterName 时需要此项以防止同名条目被丢弃。 |

## 响应

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## 示例

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 页面 URL 标识符（由服务器端清理）。
let afterName = "afterName_example" // String | 游标：传入上一响应的 nextAfterName。（可选）
let afterUserId = "afterUserId_example" // String | 游标决胜符：传入上一响应的 nextAfterUserId。当设置了 afterName 时需要此项以防止同名条目被丢弃。（可选）

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]