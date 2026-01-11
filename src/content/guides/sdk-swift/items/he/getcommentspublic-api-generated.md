req
tenantId
urlId

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| page | integer | query | לא |  |
| direction | string | query | לא |  |
| sso | string | query | לא |  |
| skip | integer | query | לא |  |
| skipChildren | integer | query | לא |  |
| limit | integer | query | לא |  |
| limitChildren | integer | query | לא |  |
| countChildren | boolean | query | לא |  |
| fetchPageForCommentId | string | query | לא |  |
| includeConfig | boolean | query | לא |  |
| countAll | boolean | query | לא |  |
| includei10n | boolean | query | לא |  |
| locale | string | query | לא |  |
| modules | string | query | לא |  |
| isCrawler | boolean | query | לא |  |
| includeNotificationCount | boolean | query | לא |  |
| asTree | boolean | query | לא |  |
| maxTreeDepth | integer | query | לא |  |
| useFullTranslationIds | boolean | query | לא |  |
| parentId | string | query | לא |  |
| searchText | string | query | לא |  |
| hashTags | array | query | לא |  |
| userId | string | query | לא |  |
| customConfigStr | string | query | לא |  |
| afterCommentId | string | query | לא |  |
| beforeCommentId | string | query | לא |  |

## תגובה

מחזיר: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד שלהלן עדיין בבטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (אופציונלי)
let direction = SortDirections() // SortDirections |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)
let skip = 987 // Int |  (אופציונלי)
let skipChildren = 987 // Int |  (אופציונלי)
let limit = 987 // Int |  (אופציונלי)
let limitChildren = 987 // Int |  (אופציונלי)
let countChildren = true // Bool |  (אופציונלי)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (אופציונלי)
let includeConfig = true // Bool |  (אופציונלי)
let countAll = true // Bool |  (אופציונלי)
let includei10n = true // Bool |  (אופציונלי)
let locale = "locale_example" // String |  (אופציונלי)
let modules = "modules_example" // String |  (אופציונלי)
let isCrawler = true // Bool |  (אופציונלי)
let includeNotificationCount = true // Bool |  (אופציונלי)
let asTree = true // Bool |  (אופציונלי)
let maxTreeDepth = 987 // Int |  (אופציונלי)
let useFullTranslationIds = true // Bool |  (אופציונלי)
let parentId = "parentId_example" // String |  (אופציונלי)
let searchText = "searchText_example" // String |  (אופציונלי)
let hashTags = ["inner_example"] // [String] |  (אופציונלי)
let userId = "userId_example" // String |  (אופציונלי)
let customConfigStr = "customConfigStr_example" // String |  (אופציונלי)
let afterCommentId = "afterCommentId_example" // String |  (אופציונלי)
let beforeCommentId = "beforeCommentId_example" // String |  (אופציונלי)

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