## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| userId | string | 查询 | 否 |  |
| badgeId | string | 查询 | 否 |  |
| type | number | 查询 | 否 |  |
| displayedOnComments | boolean | 查询 | 否 |  |
| limit | number | 查询 | 否 |  |
| skip | number | 查询 | 否 |  |

## 响应

返回: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## 示例

[inline-code-attrs-start title = 'getUserBadges 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  （可选）
let badgeId = "badgeId_example" // String |  （可选）
let type = 987 // Double |  （可选）
let displayedOnComments = true // Bool |  （可选）
let limit = 987 // Double |  （可选）
let skip = 987 // Double |  （可选）

DefaultAPI.getUserBadges(tenantId: tenantId, options: DefaultAPI.GetUserBadgesOptions(userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip)) { (response, error) in
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