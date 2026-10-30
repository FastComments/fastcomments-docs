---
識別正在使用的憑證：它所屬的租戶，以及對於 OAuth 令牌，授權該令牌的使用者。  
整合使用此資訊來測試連線並為其加上標籤。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 回應

返回：[`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## 範例

[inline-code-attrs-start title = 'getMe 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍屬測試階段。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
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