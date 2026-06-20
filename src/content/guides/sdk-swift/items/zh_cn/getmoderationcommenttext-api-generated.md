## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentTextResponse.swift)

## 示例

[inline-code-attrs-start title = 'getModerationCommentText 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。若有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (可选)

ModerationAPI.getModerationCommentText(commentId: commentId, sso: sso) { (response, error) in
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