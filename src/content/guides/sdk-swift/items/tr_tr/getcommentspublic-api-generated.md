req
tenantId
urlId

## Parametreler

| Name | Type | Location | Required | Description |
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

Döndürür: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Örnek

[inline-code-attrs-start title = 'getCommentsPublic Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hala beta durumundadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (isteğe bağlı)
let direction = SortDirections() // SortDirections |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)
let skip = 987 // Int |  (isteğe bağlı)
let skipChildren = 987 // Int |  (isteğe bağlı)
let limit = 987 // Int |  (isteğe bağlı)
let limitChildren = 987 // Int |  (isteğe bağlı)
let countChildren = true // Bool |  (isteğe bağlı)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (isteğe bağlı)
let includeConfig = true // Bool |  (isteğe bağlı)
let countAll = true // Bool |  (isteğe bağlı)
let includei10n = true // Bool |  (isteğe bağlı)
let locale = "locale_example" // String |  (isteğe bağlı)
let modules = "modules_example" // String |  (isteğe bağlı)
let isCrawler = true // Bool |  (isteğe bağlı)
let includeNotificationCount = true // Bool |  (isteğe bağlı)
let asTree = true // Bool |  (isteğe bağlı)
let maxTreeDepth = 987 // Int |  (isteğe bağlı)
let useFullTranslationIds = true // Bool |  (isteğe bağlı)
let parentId = "parentId_example" // String |  (isteğe bağlı)
let searchText = "searchText_example" // String |  (isteğe bağlı)
let hashTags = ["inner_example"] // [String] |  (isteğe bağlı)
let userId = "userId_example" // String |  (isteğe bağlı)
let customConfigStr = "customConfigStr_example" // String |  (isteğe bağlı)
let afterCommentId = "afterCommentId_example" // String |  (isteğe bağlı)
let beforeCommentId = "beforeCommentId_example" // String |  (isteğe bağlı)

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