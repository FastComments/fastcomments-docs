İstek
tenantId
urlId

## Parametreler

| Ad | Tip | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| page | integer | query | Hayır |  |
| direction | string | query | Hayır |  |
| sso | string | query | Hayır |  |
| skip | integer | query | Hayır |  |
| skipChildren | integer | query | Hayır |  |
| limit | integer | query | Hayır |  |
| limitChildren | integer | query | Hayır |  |
| countChildren | boolean | query | Hayır |  |
| fetchPageForCommentId | string | query | Hayır |  |
| includeConfig | boolean | query | Hayır |  |
| countAll | boolean | query | Hayır |  |
| includei10n | boolean | query | Hayır |  |
| locale | string | query | Hayır |  |
| modules | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeNotificationCount | boolean | query | Hayır |  |
| asTree | boolean | query | Hayır |  |
| maxTreeDepth | integer | query | Hayır |  |
| useFullTranslationIds | boolean | query | Hayır |  |
| parentId | string | query | Hayır |  |
| searchText | string | query | Hayır |  |
| hashTags | array | query | Hayır |  |
| userId | string | query | Hayır |  |
| customConfigStr | string | query | Hayır |  |
| afterCommentId | string | query | Hayır |  |
| beforeCommentId | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Örnek

[inline-code-attrs-start title = 'getCommentsPublic Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
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