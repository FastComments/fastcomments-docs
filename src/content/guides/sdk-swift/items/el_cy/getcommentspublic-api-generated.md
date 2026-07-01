req
tenantId
urlId

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
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

## Απόκριση

Επιστρέφει: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'getCommentsPublic Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
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

PublicAPI.getCommentsPublic(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetCommentsPublicOptions(page: page, direction: direction, sso: sso, skip: skip, skipChildren: skipChildren, limit: limit, limitChildren: limitChildren, countChildren: countChildren, fetchPageForCommentId: fetchPageForCommentId, includeConfig: includeConfig, countAll: countAll, includei10n: includei10n, locale: locale, modules: modules, isCrawler: isCrawler, includeNotificationCount: includeNotificationCount, asTree: asTree, maxTreeDepth: maxTreeDepth, useFullTranslationIds: useFullTranslationIds, parentId: parentId, searchText: searchText, hashTags: hashTags, userId: userId, customConfigStr: customConfigStr, afterCommentId: afterCommentId, beforeCommentId: beforeCommentId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]