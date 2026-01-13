## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| locale | string | query | 否 |  |

## 回應

回傳: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/RenderEmailTemplate200Response.swift)

## 範例

[inline-code-attrs-start title = 'renderEmailTemplate 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let renderEmailTemplateBody = RenderEmailTemplateBody(emailTemplateId: "emailTemplateId_example", ejs: "ejs_example", testData: "TODO", translationOverridesByLocale: "TODO") // RenderEmailTemplateBody | 
let locale = "locale_example" // String |  (可選)

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

---