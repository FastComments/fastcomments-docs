## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| urlId | string | query | Yes |  |
| sso | string | query | No |  |

## Response

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## 示例

[inline-code-attrs-start title = 'putReopenThread 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如遇任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (可选)

ModerationAPI.putReopenThread(urlId: urlId, sso: sso) { (response, error) in
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