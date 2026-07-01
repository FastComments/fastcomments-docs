req  
tenantId  
urlId  

## Parameters

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

## Response

Returns: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in de bètafase. Bij problemen, rapporteer via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (optioneel)
let direction = SortDirections() // SortDirections |  (optioneel)
let sso = "sso_example" // String |  (optioneel)
let skip = 987 // Int |  (optioneel)
let skipChildren = 987 // Int |  (optioneel)
let limit = 987 // Int |  (optioneel)
let limitChildren = 987 // Int |  (optioneel)
let countChildren = true // Bool |  (optioneel)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (optioneel)
let includeConfig = true // Bool |  (optioneel)
let countAll = true // Bool |  (optioneel)
let includei10n = true // Bool |  (optioneel)
let locale = "locale_example" // String |  (optioneel)
let modules = "modules_example" // String |  (optioneel)
let isCrawler = true // Bool |  (optioneel)
let includeNotificationCount = true // Bool |  (optioneel)
let asTree = true // Bool |  (optioneel)
let maxTreeDepth = 987 // Int |  (optioneel)
let useFullTranslationIds = true // Bool |  (optioneel)
let parentId = "parentId_example" // String |  (optioneel)
let searchText = "searchText_example" // String |  (optioneel)
let hashTags = ["inner_example"] // [String] |  (optioneel)
let userId = "userId_example" // String |  (optioneel)
let customConfigStr = "customConfigStr_example" // String |  (optioneel)
let afterCommentId = "afterCommentId_example" // String |  (optioneel)
let beforeCommentId = "beforeCommentId_example" // String |  (optioneel)

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