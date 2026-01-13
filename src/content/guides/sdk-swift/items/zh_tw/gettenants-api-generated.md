## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## 回應

回傳: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenants200Response.swift)

## 範例

[inline-code-attrs-start title = 'getTenants 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let meta = "meta_example" // String |  (可選)
let skip = 987 // Double |  (可選)

DefaultAPI.getTenants(tenantId: tenantId, meta: meta, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]