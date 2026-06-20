为租户批量获取用户信息。根据 userIds，返回来自 User / SSOUser 的展示信息。
由评论小部件使用，以补充通过存在事件刚刚出现的用户信息。
没有页面上下文：隐私统一强制执行（私人资料将被屏蔽）。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| ids | string | query | 是 | 以逗号分隔的 userIds。 |

## 响应

返回：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## 示例

[inline-code-attrs-start title = 'getUsersInfo 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。如遇任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | 以逗号分隔的 userIds。

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
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