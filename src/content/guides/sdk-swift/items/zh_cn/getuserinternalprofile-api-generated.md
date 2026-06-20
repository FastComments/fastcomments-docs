## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | No |  |
| sso | string | query | No |  |

## 响应

返回: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## 示例

[inline-code-attrs-start title = 'getUserInternalProfile 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。 如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let commentId = "commentId_example" // String |  (可选)
let sso = "sso_example" // String |  (可选)

ModerationAPI.getUserInternalProfile(commentId: commentId, sso: sso) { (response, error) in
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