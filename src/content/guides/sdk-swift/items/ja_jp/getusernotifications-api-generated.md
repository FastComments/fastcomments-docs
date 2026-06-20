## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| urlId | string | query | いいえ | 現在のページが購読されているかどうかを判断するために使用されます。 |
| pageSize | integer | query | いいえ |  |
| afterId | string | query | いいえ |  |
| includeContext | boolean | query | いいえ |  |
| afterCreatedAt | integer | query | いいえ |  |
| unreadOnly | boolean | query | いいえ |  |
| dmOnly | boolean | query | いいえ |  |
| noDm | boolean | query | いいえ |  |
| includeTranslations | boolean | query | いいえ |  |
| includeTenantNotifications | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## 例

[inline-code-attrs-start title = 'getUserNotifications の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 現在のページが購読されているかどうかを判断するために使用されます。 （任意）
let pageSize = 987 // Int |  （任意）
let afterId = "afterId_example" // String |  （任意）
let includeContext = true // Bool |  （任意）
let afterCreatedAt = 987 // Int64 |  （任意）
let unreadOnly = true // Bool |  （任意）
let dmOnly = true // Bool |  （任意）
let noDm = true // Bool |  （任意）
let includeTranslations = true // Bool |  （任意）
let includeTenantNotifications = true // Bool |  （任意）
let sso = "sso_example" // String |  （任意）

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