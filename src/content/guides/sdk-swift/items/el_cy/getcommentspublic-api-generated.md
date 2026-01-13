req
tenantId
urlId

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| page | integer | query | Όχι |  |
| direction | string | query | Όχι |  |
| sso | string | query | Όχι |  |
| skip | integer | query | Όχι |  |
| skipChildren | integer | query | Όχι |  |
| limit | integer | query | Όχι |  |
| limitChildren | integer | query | Όχι |  |
| countChildren | boolean | query | Όχι |  |
| fetchPageForCommentId | string | query | Όχι |  |
| includeConfig | boolean | query | Όχι |  |
| countAll | boolean | query | Όχι |  |
| includei10n | boolean | query | Όχι |  |
| locale | string | query | Όχι |  |
| modules | string | query | Όχι |  |
| isCrawler | boolean | query | Όχι |  |
| includeNotificationCount | boolean | query | Όχι |  |
| asTree | boolean | query | Όχι |  |
| maxTreeDepth | integer | query | Όχι |  |
| useFullTranslationIds | boolean | query | Όχι |  |
| parentId | string | query | Όχι |  |
| searchText | string | query | Όχι |  |
| hashTags | array | query | Όχι |  |
| userId | string | query | Όχι |  |
| customConfigStr | string | query | Όχι |  |
| afterCommentId | string | query | Όχι |  |
| beforeCommentId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε ζήτημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (προαιρετικό)
let direction = SortDirections() // SortDirections |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)
let skip = 987 // Int |  (προαιρετικό)
let skipChildren = 987 // Int |  (προαιρετικό)
let limit = 987 // Int |  (προαιρετικό)
let limitChildren = 987 // Int |  (προαιρετικό)
let countChildren = true // Bool |  (προαιρετικό)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (προαιρετικό)
let includeConfig = true // Bool |  (προαιρετικό)
let countAll = true // Bool |  (προαιρετικό)
let includei10n = true // Bool |  (προαιρετικό)
let locale = "locale_example" // String |  (προαιρετικό)
let modules = "modules_example" // String |  (προαιρετικό)
let isCrawler = true // Bool |  (προαιρετικό)
let includeNotificationCount = true // Bool |  (προαιρετικό)
let asTree = true // Bool |  (προαιρετικό)
let maxTreeDepth = 987 // Int |  (προαιρετικό)
let useFullTranslationIds = true // Bool |  (προαιρετικό)
let parentId = "parentId_example" // String |  (προαιρετικό)
let searchText = "searchText_example" // String |  (προαιρετικό)
let hashTags = ["inner_example"] // [String] |  (προαιρετικό)
let userId = "userId_example" // String |  (προαιρετικό)
let customConfigStr = "customConfigStr_example" // String |  (προαιρετικό)
let afterCommentId = "afterCommentId_example" // String |  (προαιρετικό)
let beforeCommentId = "beforeCommentId_example" // String |  (προαιρετικό)

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