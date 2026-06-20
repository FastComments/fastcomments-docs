## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | 路径 | 是 |  |
| sso | string | 查询参数 | 否 |  |

## 响应

返回: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIChildCommentsResponse.swift)

## 示例

[inline-code-attrs-start title = 'getCommentChildren 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (可选)

ModerationAPI.getCommentChildren(commentId: commentId, sso: sso) { (response, error) in
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