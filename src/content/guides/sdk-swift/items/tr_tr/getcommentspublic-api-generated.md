req
tenantId
urlId

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
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

## Yanıt

Döndürür: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Örnek

[inline-code-attrs-start title = 'getCommentsPublic Örnek'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (isteğe bağlı)
let direction = SortDirections() // SortDirections |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)
let skip = 987 // Int |  (isteğe bağlı)
let skipChildren = 987 // Int |  (isteğe bağlı)
let limit = 987 // Int |  (isteğe bağlı)
let limitChildren = 987 // Int |  (isteğe bağlı)
let countChildren = true // Bool |  (isteğe bağlı)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (isteğe bağlı)
let includeConfig = true // Bool |  (isteğe bağlı)
let countAll = true // Bool |  (isteğe bağlı)
let includei10n = true // Bool |  (isteğe bağlı)
let locale = "locale_example" // String |  (isteğe bağlı)
let modules = "modules_example" // String |  (isteğe bağlı)
let isCrawler = true // Bool |  (isteğe bağlı)
let includeNotificationCount = true // Bool |  (isteğe bağlı)
let asTree = true // Bool |  (isteğe bağlı)
let maxTreeDepth = 987 // Int |  (isteğe bağlı)
let useFullTranslationIds = true // Bool |  (isteğe bağlı)
let parentId = "parentId_example" // String |  (isteğe bağlı)
let searchText = "searchText_example" // String |  (isteğe bağlı)
let hashTags = ["inner_example"] // [String] |  (isteğe bağlı)
let userId = "userId_example" // String |  (isteğe bağlı)
let customConfigStr = "customConfigStr_example" // String |  (isteğe bağlı)
let afterCommentId = "afterCommentId_example" // String |  (isteğe bağlı)
let beforeCommentId = "beforeCommentId_example" // String |  (isteğe bağlı)

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