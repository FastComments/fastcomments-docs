## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| pageSize | integer | query | 否 |  |
| afterId | string | query | 否 |  |
| includeContext | boolean | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| includeTranslations | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserNotifications200Response.swift)

## 範例

[inline-code-attrs-start title = 'getUserNotifications 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為 beta。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let pageSize = 987 // Int |  (可選)
let afterId = "afterId_example" // String |  (可選)
let includeContext = true // Bool |  (可選)
let afterCreatedAt = 987 // Int64 |  (可選)
let unreadOnly = true // Bool |  (可選)
let dmOnly = true // Bool |  (可選)
let noDm = true // Bool |  (可選)
let includeTranslations = true // Bool |  (可選)
let sso = "sso_example" // String |  (可選)

PublicAPI.getUserNotifications(tenantId: tenantId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]