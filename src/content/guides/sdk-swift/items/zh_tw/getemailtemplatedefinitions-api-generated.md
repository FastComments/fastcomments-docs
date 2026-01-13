## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplateDefinitions200Response.swift)

## 範例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // 字串 | 

DefaultAPI.getEmailTemplateDefinitions(tenantId: tenantId) { (response, error) in
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