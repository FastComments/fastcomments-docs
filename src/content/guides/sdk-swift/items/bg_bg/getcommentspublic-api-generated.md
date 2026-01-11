заявка
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

Връща: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета версия. За проблеми, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (по избор)
let direction = SortDirections() // SortDirections |  (по избор)
let sso = "sso_example" // String |  (по избор)
let skip = 987 // Int |  (по избор)
let skipChildren = 987 // Int |  (по избор)
let limit = 987 // Int |  (по избор)
let limitChildren = 987 // Int |  (по избор)
let countChildren = true // Bool |  (по избор)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (по избор)
let includeConfig = true // Bool |  (по избор)
let countAll = true // Bool |  (по избор)
let includei10n = true // Bool |  (по избор)
let locale = "locale_example" // String |  (по избор)
let modules = "modules_example" // String |  (по избор)
let isCrawler = true // Bool |  (по избор)
let includeNotificationCount = true // Bool |  (по избор)
let asTree = true // Bool |  (по избор)
let maxTreeDepth = 987 // Int |  (по избор)
let useFullTranslationIds = true // Bool |  (по избор)
let parentId = "parentId_example" // String |  (по избор)
let searchText = "searchText_example" // String |  (по избор)
let hashTags = ["inner_example"] // [String] |  (по избор)
let userId = "userId_example" // String |  (по избор)
let customConfigStr = "customConfigStr_example" // String |  (по избор)
let afterCommentId = "afterCommentId_example" // String |  (по избор)
let beforeCommentId = "beforeCommentId_example" // String |  (по избор)

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