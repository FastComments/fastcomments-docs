## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | number | query | 否 |  |

## 响应

返回：[`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionConfigsResponse.swift)

## 示例

[inline-code-attrs-start title = 'getQuestionConfigs 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列代码示例仍处于测试阶段。如遇任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let skip = 987 // Double |  (可选)

DefaultAPI.getQuestionConfigs(tenantId: tenantId, skip: skip) { (response, error) in
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