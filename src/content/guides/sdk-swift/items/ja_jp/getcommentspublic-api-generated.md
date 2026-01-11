req
tenantId
urlId

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| page | integer | query | いいえ |  |
| direction | string | query | いいえ |  |
| sso | string | query | いいえ |  |
| skip | integer | query | いいえ |  |
| skipChildren | integer | query | いいえ |  |
| limit | integer | query | いいえ |  |
| limitChildren | integer | query | いいえ |  |
| countChildren | boolean | query | いいえ |  |
| fetchPageForCommentId | string | query | いいえ |  |
| includeConfig | boolean | query | いいえ |  |
| countAll | boolean | query | いいえ |  |
| includei10n | boolean | query | いいえ |  |
| locale | string | query | いいえ |  |
| modules | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |
| includeNotificationCount | boolean | query | いいえ |  |
| asTree | boolean | query | いいえ |  |
| maxTreeDepth | integer | query | いいえ |  |
| useFullTranslationIds | boolean | query | いいえ |  |
| parentId | string | query | いいえ |  |
| searchText | string | query | いいえ |  |
| hashTags | array | query | いいえ |  |
| userId | string | query | いいえ |  |
| customConfigStr | string | query | いいえ |  |
| afterCommentId | string | query | いいえ |  |
| beforeCommentId | string | query | いいえ |  |

## レスポンス

返却値: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## 例

[inline-code-attrs-start title = 'getCommentsPublic の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  （オプション）
let direction = SortDirections() // SortDirections |  （オプション）
let sso = "sso_example" // String |  （オプション）
let skip = 987 // Int |  （オプション）
let skipChildren = 987 // Int |  （オプション）
let limit = 987 // Int |  （オプション）
let limitChildren = 987 // Int |  （オプション）
let countChildren = true // Bool |  （オプション）
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  （オプション）
let includeConfig = true // Bool |  （オプション）
let countAll = true // Bool |  （オプション）
let includei10n = true // Bool |  （オプション）
let locale = "locale_example" // String |  （オプション）
let modules = "modules_example" // String |  （オプション）
let isCrawler = true // Bool |  （オプション）
let includeNotificationCount = true // Bool |  （オプション）
let asTree = true // Bool |  （オプション）
let maxTreeDepth = 987 // Int |  （オプション）
let useFullTranslationIds = true // Bool |  （オプション）
let parentId = "parentId_example" // String |  （オプション）
let searchText = "searchText_example" // String |  （オプション）
let hashTags = ["inner_example"] // [String] |  （オプション）
let userId = "userId_example" // String |  （オプション）
let customConfigStr = "customConfigStr_example" // String |  （オプション）
let afterCommentId = "afterCommentId_example" // String |  （オプション）
let beforeCommentId = "beforeCommentId_example" // String |  （オプション）

PublicAPI.getCommentsPublic(tenantId: tenantId, urlId: urlId, page: page, direction: direction, sso: sso, skip: skip, skipChildren: skipChildren, limit: limit, limitChildren: limitChildren, countChildren: countChildren, fetchPageForCommentId: fetchPageForCommentId, includeConfig: includeConfig, countAll: countAll, includei10n: includei10n, locale: locale, modules: modules, isCrawler: isCrawler, includeNotificationCount: includeNotificationCount, asTree: asTree, maxTreeDepth: maxTreeDepth, useFullTranslationIds: useFullTranslationIds, parentId: parentId, searchText: searchText, hashTags: hashTags, userId: userId, customConfigStr: customConfigStr, afterCommentId: afterCommentId, beforeCommentId: beforeCommentId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]