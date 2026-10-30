req
tenantId
urlId

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
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

## תגובה

מחזיר: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## דוגמה

[inline-code-attrs-start title = 'getCommentsPublic דוגמה'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
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
let questionId = "questionId_example" // String |  (אופציונלי)
let questionValue = 987 // Int |  (אופציונלי)

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