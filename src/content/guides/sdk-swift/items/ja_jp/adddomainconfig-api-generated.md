---
## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## レスポンス

戻り値: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddDomainConfigResponse.swift)

## 例

[inline-code-attrs-start title = 'addDomainConfig の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let addDomainConfigParams = AddDomainConfigParams(domain: "domain_example", emailFromName: "emailFromName_example", emailFromEmail: "emailFromEmail_example", logoSrc: "logoSrc_example", logoSrc100px: "logoSrc100px_example", footerUnsubscribeURL: "footerUnsubscribeURL_example", emailHeaders: "TODO") // AddDomainConfigParams | 

DefaultAPI.addDomainConfig(tenantId: tenantId, addDomainConfigParams: addDomainConfigParams) { (response, error) in
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