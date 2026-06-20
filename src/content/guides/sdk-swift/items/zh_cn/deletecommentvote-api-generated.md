## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| commentId | string | 路径 | 是 |  |
| voteId | string | 路径 | 是 |  |
| urlId | string | 查询 | 是 |  |
| broadcastId | string | 查询 | 是 |  |
| editKey | string | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回：[`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteDeleteResponse.swift)

## 示例

[inline-code-attrs-start title = 'deleteCommentVote 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voteId = "voteId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let editKey = "editKey_example" // String |  (可选)
let sso = "sso_example" // String |  (可选)

PublicAPI.deleteCommentVote(tenantId: tenantId, commentId: commentId, voteId: voteId, urlId: urlId, broadcastId: broadcastId, editKey: editKey, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]