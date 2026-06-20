## 参数

| 名称 | 类型 | 位置 | 是否必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplateResponse.swift)

## 示例

[inline-code-attrs-start title = 'getEmailTemplate 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下面的代码示例仍处于测试阶段。如有问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.getEmailTemplate(tenantId: tenantId, id: id) { (response, error) in
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