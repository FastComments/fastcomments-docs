启用或禁用针对特定评论的通知。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| notificationId | string | 路径 | 是 |  |
| optedInOrOut | string | 路径 | 是 |  |
| commentId | string | 查询 | 是 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回：[`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationCommentSubscriptionStatusResponse.swift)

## 示例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。如遇问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // 字符串 | 
let notificationId = "notificationId_example" // 字符串 | 
let optedInOrOut = "optedInOrOut_example" // 字符串 | 
let commentId = "commentId_example" // 字符串 | 
let sso = "sso_example" // 字符串 |  (可选)

PublicAPI.updateUserNotificationCommentSubscriptionStatus(tenantId: tenantId, notificationId: notificationId, optedInOrOut: optedInOrOut, commentId: commentId, sso: sso) { (response, error) in
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