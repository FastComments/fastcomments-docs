req
tenantId
urlId

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
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

Vraća: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Primer

[inline-code-attrs-start title = 'getCommentsPublic Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek beta. Za bilo koji problem, prijavite preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (opciono)
let direction = SortDirections() // SortDirections |  (opciono)
let sso = "sso_example" // String |  (opciono)
let skip = 987 // Int |  (opciono)
let skipChildren = 987 // Int |  (opciono)
let limit = 987 // Int |  (opciono)
let limitChildren = 987 // Int |  (opciono)
let countChildren = true // Bool |  (opciono)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (opciono)
let includeConfig = true // Bool |  (opciono)
let countAll = true // Bool |  (opciono)
let includei10n = true // Bool |  (opciono)
let locale = "locale_example" // String |  (opciono)
let modules = "modules_example" // String |  (opciono)
let isCrawler = true // Bool |  (opciono)
let includeNotificationCount = true // Bool |  (opciono)
let asTree = true // Bool |  (opciono)
let maxTreeDepth = 987 // Int |  (opciono)
let useFullTranslationIds = true // Bool |  (opciono)
let parentId = "parentId_example" // String |  (opciono)
let searchText = "searchText_example" // String |  (opciono)
let hashTags = ["inner_example"] // [String] |  (opciono)
let userId = "userId_example" // String |  (opciono)
let customConfigStr = "customConfigStr_example" // String |  (opciono)
let afterCommentId = "afterCommentId_example" // String |  (opciono)
let beforeCommentId = "beforeCommentId_example" // String |  (opciono)

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