req
tenantId
urlId

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| page | integer | query | Не |  |
| direction | string | query | Не |  |
| sso | string | query | Не |  |
| skip | integer | query | Не |  |
| skipChildren | integer | query | Не |  |
| limit | integer | query | Не |  |
| limitChildren | integer | query | Не |  |
| countChildren | boolean | query | Не |  |
| fetchPageForCommentId | string | query | Не |  |
| includeConfig | boolean | query | Не |  |
| countAll | boolean | query | Не |  |
| includei10n | boolean | query | Не |  |
| locale | string | query | Не |  |
| modules | string | query | Не |  |
| isCrawler | boolean | query | Не |  |
| includeNotificationCount | boolean | query | Не |  |
| asTree | boolean | query | Не |  |
| maxTreeDepth | integer | query | Не |  |
| useFullTranslationIds | boolean | query | Не |  |
| parentId | string | query | Не |  |
| searchText | string | query | Не |  |
| hashTags | array | query | Не |  |
| userId | string | query | Не |  |
| customConfigStr | string | query | Не |  |
| afterCommentId | string | query | Не |  |
| beforeCommentId | string | query | Не |  |

## Отговор

Връща: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Пример

[inline-code-attrs-start title = 'getCommentsPublic Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни кодове все още са в бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (незадължително)
let direction = SortDirections() // SortDirections |  (незадължително)
let sso = "sso_example" // String |  (незадължително)
let skip = 987 // Int |  (незадължително)
let skipChildren = 987 // Int |  (незадължително)
let limit = 987 // Int |  (незадължително)
let limitChildren = 987 // Int |  (незадължително)
let countChildren = true // Bool |  (незадължително)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (незадължително)
let includeConfig = true // Bool |  (незадължително)
let countAll = true // Bool |  (незадължително)
let includei10n = true // Bool |  (незадължително)
let locale = "locale_example" // String |  (незадължително)
let modules = "modules_example" // String |  (незадължително)
let isCrawler = true // Bool |  (незадължително)
let includeNotificationCount = true // Bool |  (незадължително)
let asTree = true // Bool |  (незадължително)
let maxTreeDepth = 987 // Int |  (незадължително)
let useFullTranslationIds = true // Bool |  (незадължително)
let parentId = "parentId_example" // String |  (незадължително)
let searchText = "searchText_example" // String |  (незадължително)
let hashTags = ["inner_example"] // [String] |  (незадължително)
let userId = "userId_example" // String |  (незадължително)
let customConfigStr = "customConfigStr_example" // String |  (незадължително)
let afterCommentId = "afterCommentId_example" // String |  (незадължително)
let beforeCommentId = "beforeCommentId_example" // String |  (незадължително)

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