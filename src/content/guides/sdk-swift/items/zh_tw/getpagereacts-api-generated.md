自訂頁面的反應計數，可選擇包含做出反應的使用者。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |
| includeUserInfo | string | query | 否 | "public" 會省略私人個人資料的使用者，"include-private" 則會包含他們。 |

## 回應

回傳：[`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageReactsAPIResponse.swift)

## 範例

[inline-code-attrs-start title = 'getPageReacts 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" 會省略私人個人資料的使用者，\"include-private\" 則會包含他們。(optional)

DefaultAPI.getPageReacts(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]