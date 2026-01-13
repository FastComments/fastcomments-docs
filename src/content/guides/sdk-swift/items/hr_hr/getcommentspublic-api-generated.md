req
tenantId
urlId

## Parametri

| Name | Type | Location | Required | Description |
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

## Primjer

[inline-code-attrs-start title = 'getCommentsPublic Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (neobavezno)
let direction = SortDirections() // SortDirections |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)
let skip = 987 // Int |  (neobavezno)
let skipChildren = 987 // Int |  (neobavezno)
let limit = 987 // Int |  (neobavezno)
let limitChildren = 987 // Int |  (neobavezno)
let countChildren = true // Bool |  (neobavezno)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (neobavezno)
let includeConfig = true // Bool |  (neobavezno)
let countAll = true // Bool |  (neobavezno)
let includei10n = true // Bool |  (neobavezno)
let locale = "locale_example" // String |  (neobavezno)
let modules = "modules_example" // String |  (neobavezno)
let isCrawler = true // Bool |  (neobavezno)
let includeNotificationCount = true // Bool |  (neobavezno)
let asTree = true // Bool |  (neobavezno)
let maxTreeDepth = 987 // Int |  (neobavezno)
let useFullTranslationIds = true // Bool |  (neobavezno)
let parentId = "parentId_example" // String |  (neobavezno)
let searchText = "searchText_example" // String |  (neobavezno)
let hashTags = ["inner_example"] // [String] |  (neobavezno)
let userId = "userId_example" // String |  (neobavezno)
let customConfigStr = "customConfigStr_example" // String |  (neobavezno)
let afterCommentId = "afterCommentId_example" // String |  (neobavezno)
let beforeCommentId = "beforeCommentId_example" // String |  (neobavezno)

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