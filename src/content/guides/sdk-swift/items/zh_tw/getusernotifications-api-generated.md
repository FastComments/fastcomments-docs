## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 | 用於判斷當前頁面是否已訂閱。 |
| pageSize | integer | query | 否 |  |
| afterId | string | query | 否 |  |
| includeContext | boolean | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| includeTranslations | boolean | query | 否 |  |
| includeTenantNotifications | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回傳

回傳: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## 範例

[inline-code-attrs-start title = 'getUserNotifications 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為測試版。如遇任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 用於判斷當前頁面是否已訂閱。 (選用)
let pageSize = 987 // Int |  (選用)
let afterId = "afterId_example" // String |  (選用)
let includeContext = true // Bool |  (選用)
let afterCreatedAt = 987 // Int64 |  (選用)
let unreadOnly = true // Bool |  (選用)
let dmOnly = true // Bool |  (選用)
let noDm = true // Bool |  (選用)
let includeTranslations = true // Bool |  (選用)
let includeTenantNotifications = true // Bool |  (選用)
let sso = "sso_example" // String |  (選用)

PublicAPI.getUserNotifications(tenantId: tenantId, urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]