req
tenantId
urlId

## Parametri

| Name | Type | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| page | integer | query | Ne |  |
| direction | string | query | Ne |  |
| sso | string | query | Ne |  |
| skip | integer | query | Ne |  |
| skipChildren | integer | query | Ne |  |
| limit | integer | query | Ne |  |
| limitChildren | integer | query | Ne |  |
| countChildren | boolean | query | Ne |  |
| fetchPageForCommentId | string | query | Ne |  |
| includeConfig | boolean | query | Ne |  |
| countAll | boolean | query | Ne |  |
| includei10n | boolean | query | Ne |  |
| locale | string | query | Ne |  |
| modules | string | query | Ne |  |
| isCrawler | boolean | query | Ne |  |
| includeNotificationCount | boolean | query | Ne |  |
| asTree | boolean | query | Ne |  |
| maxTreeDepth | integer | query | Ne |  |
| useFullTranslationIds | boolean | query | Ne |  |
| parentId | string | query | Ne |  |
| searchText | string | query | Ne |  |
| hashTags | array | query | Ne |  |
| userId | string | query | Ne |  |
| customConfigStr | string | query | Ne |  |
| afterCommentId | string | query | Ne |  |
| beforeCommentId | string | query | Ne |  |

## Odgovor

Vrne: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za kakršnokoli težavo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (neobvezno)
let direction = SortDirections() // SortDirections |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)
let skip = 987 // Int |  (neobvezno)
let skipChildren = 987 // Int |  (neobvezno)
let limit = 987 // Int |  (neobvezno)
let limitChildren = 987 // Int |  (neobvezno)
let countChildren = true // Bool |  (neobvezno)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (neobvezno)
let includeConfig = true // Bool |  (neobvezno)
let countAll = true // Bool |  (neobvezno)
let includei10n = true // Bool |  (neobvezno)
let locale = "locale_example" // String |  (neobvezno)
let modules = "modules_example" // String |  (neobvezno)
let isCrawler = true // Bool |  (neobvezno)
let includeNotificationCount = true // Bool |  (neobvezno)
let asTree = true // Bool |  (neobvezno)
let maxTreeDepth = 987 // Int |  (neobvezno)
let useFullTranslationIds = true // Bool |  (neobvezno)
let parentId = "parentId_example" // String |  (neobvezno)
let searchText = "searchText_example" // String |  (neobvezno)
let hashTags = ["inner_example"] // [String] |  (neobvezno)
let userId = "userId_example" // String |  (neobvezno)
let customConfigStr = "customConfigStr_example" // String |  (neobvezno)
let afterCommentId = "afterCommentId_example" // String |  (neobvezno)
let beforeCommentId = "beforeCommentId_example" // String |  (neobvezno)

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