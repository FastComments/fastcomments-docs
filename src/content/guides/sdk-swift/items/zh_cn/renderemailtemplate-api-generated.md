## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| locale | string | query | 否 |  |

## 响应

返回: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/RenderEmailTemplate200Response.swift)

## 示例

[inline-code-attrs-start title = 'renderEmailTemplate 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let renderEmailTemplateBody = RenderEmailTemplateBody(emailTemplateId: "emailTemplateId_example", ejs: "ejs_example", testData: "TODO", translationOverridesByLocale: "TODO") // RenderEmailTemplateBody | 
let locale = "locale_example" // String |  (可选)

DefaultAPI.renderEmailTemplate(tenantId: tenantId, renderEmailTemplateBody: renderEmailTemplateBody, locale: locale) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]