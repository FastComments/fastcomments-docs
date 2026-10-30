req
tenantId
urlId

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| page | integer | query | No |  |
| direction | string | query | No |  |
| sso | string | query | No |  |
| skip | integer | query | No |  |
| skipChildren | integer | query | No |  |
| limit | integer | query | No |  |
| limitChildren | integer | query | No |  |
| countChildren | boolean | query | No |  |
| fetchPageForCommentId | string | query | No |  |
| includeConfig | boolean | query | No |  |
| countAll | boolean | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| modules | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeNotificationCount | boolean | query | No |  |
| asTree | boolean | query | No |  |
| maxTreeDepth | integer | query | No |  |
| useFullTranslationIds | boolean | query | No |  |
| parentId | string | query | No |  |
| searchText | string | query | No |  |
| hashTags | array | query | No |  |
| userId | string | query | No |  |
| customConfigStr | string | query | No |  |
| afterCommentId | string | query | No |  |
| beforeCommentId | string | query | No |  |
| questionId | string | query | No |  |
| questionValue | integer | query | No |  |

## レスポンス

返却: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## 例

[inline-code-attrs-start title = 'getCommentsPublic の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (オプション)
let direction = SortDirections() // SortDirections |  (オプション)
let sso = "sso_example" // String |  (オプション)
let skip = 987 // Int |  (オプション)
let skipChildren = 987 // Int |  (オプション)
let limit = 987 // Int |  (オプション)
let limitChildren = 987 // Int |  (オプション)
let countChildren = true // Bool |  (オプション)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (オプション)
let includeConfig = true // Bool |  (オプション)
let countAll = true // Bool |  (オプション)
let includei10n = true // Bool |  (オプション)
let locale = "locale_example" // String |  (オプション)
let modules = "modules_example" // String |  (オプション)
let isCrawler = true // Bool |  (オプション)
let includeNotificationCount = true // Bool |  (オプション)
let asTree = true // Bool |  (オプション)
let maxTreeDepth = 987 // Int |  (オプション)
let useFullTranslationIds = true // Bool |  (オプション)
let parentId = "parentId_example" // String |  (オプション)
let searchText = "searchText_example" // String |  (オプション)
let hashTags = ["inner_example"] // [String] |  (オプション)
let userId = "userId_example" // String |  (オプション)
let customConfigStr = "customConfigStr_example" // String |  (オプション)
let afterCommentId = "afterCommentId_example" // String |  (オプション)
let beforeCommentId = "beforeCommentId_example" // String |  (オプション)
let questionId = "questionId_example" // String |  (オプション)
let questionValue = 987 // Int |  (オプション)

PublicAPI.getCommentsPublic(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetCommentsPublicOptions(page: page, direction: direction, sso: sso, skip: skip, skipChildren: skipChildren, limit: limit, limitChildren: limitChildren, countChildren: countChildren, fetchPageForCommentId: fetchPageForCommentId, includeConfig: includeConfig, countAll: countAll, includei10n: includei10n, locale: locale, modules: modules, isCrawler: isCrawler, includeNotificationCount: includeNotificationCount, asTree: asTree, maxTreeDepth: maxTreeDepth, useFullTranslationIds: useFullTranslationIds, parentId: parentId, searchText: searchText, hashTags: hashTags, userId: userId, customConfigStr: customConfigStr, afterCommentId: afterCommentId, beforeCommentId: beforeCommentId, questionId: questionId, questionValue: questionValue)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]