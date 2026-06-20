## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| reviewed | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## 示例

[inline-code-attrs-start title = 'postSetCommentReviewStatus 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如遇问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let reviewed = true // Bool |  (可选)
let sso = "sso_example" // String |  (可选)

ModerationAPI.postSetCommentReviewStatus(commentId: commentId, reviewed: reviewed, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]