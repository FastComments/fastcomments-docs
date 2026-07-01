req  
tenantId  
urlId  

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
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

## Odgovor

Returns: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Primjer

[inline-code-attrs-start title = 'getCommentsPublic Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su i dalje beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (opcionalno)
let direction = SortDirections() // SortDirections |  (opcionalno)
let sso = "sso_example" // String |  (opcionalno)
let skip = 987 // Int |  (opcionalno)
let skipChildren = 987 // Int |  (opcionalno)
let limit = 987 // Int |  (opcionalno)
let limitChildren = 987 // Int |  (opcionalno)
let countChildren = true // Bool |  (opcionalno)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (opcionalno)
let includeConfig = true // Bool |  (opcionalno)
let countAll = true // Bool |  (opcionalno)
let includei10n = true // Bool |  (opcionalno)
let locale = "locale_example" // String |  (opcionalno)
let modules = "modules_example" // String |  (opcionalno)
let isCrawler = true // Bool |  (opcionalno)
let includeNotificationCount = true // Bool |  (opcionalno)
let asTree = true // Bool |  (opcionalno)
let maxTreeDepth = 987 // Int |  (opcionalno)
let useFullTranslationIds = true // Bool |  (opcionalno)
let parentId = "parentId_example" // String |  (opcionalno)
let searchText = "searchText_example" // String |  (opcionalno)
let hashTags = ["inner_example"] // [String] |  (opcionalno)
let userId = "userId_example" // String |  (opcionalno)
let customConfigStr = "customConfigStr_example" // String |  (opcionalno)
let afterCommentId = "afterCommentId_example" // String |  (opcionalno)
let beforeCommentId = "beforeCommentId_example" // String |  (opcionalno)

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