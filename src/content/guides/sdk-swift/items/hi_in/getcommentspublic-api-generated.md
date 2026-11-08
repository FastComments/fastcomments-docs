req
tenantId
urlId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| direction | string | query | नहीं |  |
| sso | string | query | नहीं |  |
| skip | integer | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| countChildren | boolean | query | नहीं |  |
| fetchPageForCommentId | string | query | नहीं |  |
| includeConfig | boolean | query | नहीं |  |
| countAll | boolean | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| modules | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |
| includeNotificationCount | boolean | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| useFullTranslationIds | boolean | query | नहीं |  |
| parentId | string | query | नहीं |  |
| searchText | string | query | नहीं |  |
| hashTags | array | query | नहीं |  |
| userId | string | query | नहीं |  |
| customConfigStr | string | query | नहीं |  |
| afterCommentId | string | query | नहीं |  |
| beforeCommentId | string | query | नहीं |  |
| questionId | string | query | नहीं |  |
| questionValue | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## उदाहरण

[inline-code-attrs-start title = 'getCommentsPublic उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new के माध्यम से रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (optional)
let direction = SortDirections() // SortDirections |  (optional)
let sso = "sso_example" // String |  (optional)
let skip = 987 // Int |  (optional)
let skipChildren = 987 // Int |  (optional)
let limit = 987 // Int |  (optional)
let limitChildren = 987 // Int |  (optional)
let countChildren = true // Bool |  (optional)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (optional)
let includeConfig = true // Bool |  (optional)
let countAll = true // Bool |  (optional)
let includei10n = true // Bool |  (optional)
let locale = "locale_example" // String |  (optional)
let modules = "modules_example" // String |  (optional)
let isCrawler = true // Bool |  (optional)
let includeNotificationCount = true // Bool |  (optional)
let asTree = true // Bool |  (optional)
let maxTreeDepth = 987 // Int |  (optional)
let useFullTranslationIds = true // Bool |  (optional)
let parentId = "parentId_example" // String |  (optional)
let searchText = "searchText_example" // String |  (optional)
let hashTags = ["inner_example"] // [String] |  (optional)
let userId = "userId_example" // String |  (optional)
let customConfigStr = "customConfigStr_example" // String |  (optional)
let afterCommentId = "afterCommentId_example" // String |  (optional)
let beforeCommentId = "beforeCommentId_example" // String |  (optional)
let questionId = "questionId_example" // String |  (optional)
let questionValue = 987 // Int |  (optional)

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