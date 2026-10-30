## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | 設定後，返回每個 react id 的使用者（頭像和使用者名稱）。僅接受 "public"，會省略私人個人檔案。 |

## 回應

返回：[`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## 範例

[inline-code-attrs-start title = 'getV2PageReacts 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍屬於測試階段。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | 設定後，返回每個 react id 的使用者（頭像和使用者名稱）。僅接受 \"public\"，會省略私人個人檔案。 (optional)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]