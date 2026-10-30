req
tenantId
urlId

## Parametri

| Name | Type | Location | Required | Description |
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

## Odziv

Vrne: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Primer

[inline-code-attrs-start title = 'getCommentsPublic Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta različici. Za kakršnekoli težave, prosimo, poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (neobvezno)
let direction = SortDirections() // SortDirections |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)
let skip = 987 // Int |  (neobvezno)
let skipChildren = 987 // Int |  (neobvezno)
let limit = 987 // Int |  (neobvezno)
let limitChildren = 987 // Int |  (neobvezno)
let countChildren = true // Bool |  (neobvezno)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (neobvezno)
let includeConfig = true // Bool |  (neobvezno)
let countAll = true // Bool |  (neobvezno)
let includei10n = true // Bool |  (neobvezno)
let locale = "locale_example" // String |  (neobvezno)
let modules = "modules_example" // String |  (neobvezno)
let isCrawler = true // Bool |  (neobvezno)
let includeNotificationCount = true // Bool |  (neobvezno)
let asTree = true // Bool |  (neobvezno)
let maxTreeDepth = 987 // Int |  (neobvezno)
let useFullTranslationIds = true // Bool |  (neobvezno)
let parentId = "parentId_example" // String |  (neobvezno)
let searchText = "searchText_example" // String |  (neobvezno)
let hashTags = ["inner_example"] // [String] |  (neobvezno)
let userId = "userId_example" // String |  (neobvezno)
let customConfigStr = "customConfigStr_example" // String |  (neobvezno)
let afterCommentId = "afterCommentId_example" // String |  (neobvezno)
let beforeCommentId = "beforeCommentId_example" // String |  (neobvezno)
let questionId = "questionId_example" // String |  (neobvezno)
let questionValue = 987 // Int |  (neobvezno)

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