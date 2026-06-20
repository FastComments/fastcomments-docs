req
tenantId
urlId

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| page | integer | query | Nie |  |
| direction | string | query | Nie |  |
| sso | string | query | Nie |  |
| skip | integer | query | Nie |  |
| skipChildren | integer | query | Nie |  |
| limit | integer | query | Nie |  |
| limitChildren | integer | query | Nie |  |
| countChildren | boolean | query | Nie |  |
| fetchPageForCommentId | string | query | Nie |  |
| includeConfig | boolean | query | Nie |  |
| countAll | boolean | query | Nie |  |
| includei10n | boolean | query | Nie |  |
| locale | string | query | Nie |  |
| modules | string | query | Nie |  |
| isCrawler | boolean | query | Nie |  |
| includeNotificationCount | boolean | query | Nie |  |
| asTree | boolean | query | Nie |  |
| maxTreeDepth | integer | query | Nie |  |
| useFullTranslationIds | boolean | query | Nie |  |
| parentId | string | query | Nie |  |
| searchText | string | query | Nie |  |
| hashTags | array | query | Nie |  |
| userId | string | query | Nie |  |
| customConfigStr | string | query | Nie |  |
| afterCommentId | string | query | Nie |  |
| beforeCommentId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemu zgłoś go za pomocą http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (opcjonalne)
let direction = SortDirections() // SortDirections |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)
let skip = 987 // Int |  (opcjonalne)
let skipChildren = 987 // Int |  (opcjonalne)
let limit = 987 // Int |  (opcjonalne)
let limitChildren = 987 // Int |  (opcjonalne)
let countChildren = true // Bool |  (opcjonalne)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (opcjonalne)
let includeConfig = true // Bool |  (opcjonalne)
let countAll = true // Bool |  (opcjonalne)
let includei10n = true // Bool |  (opcjonalne)
let locale = "locale_example" // String |  (opcjonalne)
let modules = "modules_example" // String |  (opcjonalne)
let isCrawler = true // Bool |  (opcjonalne)
let includeNotificationCount = true // Bool |  (opcjonalne)
let asTree = true // Bool |  (opcjonalne)
let maxTreeDepth = 987 // Int |  (opcjonalne)
let useFullTranslationIds = true // Bool |  (opcjonalne)
let parentId = "parentId_example" // String |  (opcjonalne)
let searchText = "searchText_example" // String |  (opcjonalne)
let hashTags = ["inner_example"] // [String] |  (opcjonalne)
let userId = "userId_example" // String |  (opcjonalne)
let customConfigStr = "customConfigStr_example" // String |  (opcjonalne)
let afterCommentId = "afterCommentId_example" // String |  (opcjonalne)
let beforeCommentId = "beforeCommentId_example" // String |  (opcjonalne)

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