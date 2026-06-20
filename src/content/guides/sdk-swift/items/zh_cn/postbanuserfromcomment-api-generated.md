## 参数

| 名称 | 类型 | 位置 | 必需 | 说明 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| banEmail | boolean | query | 否 |  |
| banEmailDomain | boolean | query | 否 |  |
| banIP | boolean | query | 否 |  |
| deleteAllUsersComments | boolean | query | 否 |  |
| bannedUntil | string | query | 否 |  |
| isShadowBan | boolean | query | 否 |  |
| updateId | string | query | 否 |  |
| banReason | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回：[`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## 示例

[inline-code-attrs-start title = 'postBanUserFromComment 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (可选)
let banEmailDomain = true // Bool |  (可选)
let banIP = true // Bool |  (可选)
let deleteAllUsersComments = true // Bool |  (可选)
let bannedUntil = "bannedUntil_example" // String |  (可选)
let isShadowBan = true // Bool |  (可选)
let updateId = "updateId_example" // String |  (可选)
let banReason = "banReason_example" // String |  (可选)
let sso = "sso_example" // String |  (可选)

ModerationAPI.postBanUserFromComment(commentId: commentId, banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]