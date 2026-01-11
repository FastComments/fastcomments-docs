req
tenantId
urlId

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | ścieżka | Tak |  |
| urlId | string | zapytanie | Tak |  |
| page | integer | zapytanie | Nie |  |
| direction | string | zapytanie | Nie |  |
| sso | string | zapytanie | Nie |  |
| skip | integer | zapytanie | Nie |  |
| skipChildren | integer | zapytanie | Nie |  |
| limit | integer | zapytanie | Nie |  |
| limitChildren | integer | zapytanie | Nie |  |
| countChildren | boolean | zapytanie | Nie |  |
| fetchPageForCommentId | string | zapytanie | Nie |  |
| includeConfig | boolean | zapytanie | Nie |  |
| countAll | boolean | zapytanie | Nie |  |
| includei10n | boolean | zapytanie | Nie |  |
| locale | string | zapytanie | Nie |  |
| modules | string | zapytanie | Nie |  |
| isCrawler | boolean | zapytanie | Nie |  |
| includeNotificationCount | boolean | zapytanie | Nie |  |
| asTree | boolean | zapytanie | Nie |  |
| maxTreeDepth | integer | zapytanie | Nie |  |
| useFullTranslationIds | boolean | zapytanie | Nie |  |
| parentId | string | zapytanie | Nie |  |
| searchText | string | zapytanie | Nie |  |
| hashTags | array | zapytanie | Nie |  |
| userId | string | zapytanie | Nie |  |
| customConfigStr | string | zapytanie | Nie |  |
| afterCommentId | string | zapytanie | Nie |  |
| beforeCommentId | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemu, prosimy zgłosić go przez http://github.com/OpenAPITools/openapi-generator/issues/new
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