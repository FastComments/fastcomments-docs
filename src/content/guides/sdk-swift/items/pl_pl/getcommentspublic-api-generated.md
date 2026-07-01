req
tenantId
urlId

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|------------|----------|------|
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

[inline-code-attrs-start title = 'getCommentsPublic Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W przypadku jakichkolwiek problemów, proszę zgłaszać je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (opcjonalny)
let direction = SortDirections() // SortDirections |  (opcjonalny)
let sso = "sso_example" // String |  (opcjonalny)
let skip = 987 // Int |  (opcjonalny)
let skipChildren = 987 // Int |  (opcjonalny)
let limit = 987 // Int |  (opcjonalny)
let limitChildren = 987 // Int |  (opcjonalny)
let countChildren = true // Bool |  (opcjonalny)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (opcjonalny)
let includeConfig = true // Bool |  (opcjonalny)
let countAll = true // Bool |  (opcjonalny)
let includei10n = true // Bool |  (opcjonalny)
let locale = "locale_example" // String |  (opcjonalny)
let modules = "modules_example" // String |  (opcjonalny)
let isCrawler = true // Bool |  (opcjonalny)
let includeNotificationCount = true // Bool |  (opcjonalny)
let asTree = true // Bool |  (opcjonalny)
let maxTreeDepth = 987 // Int |  (opcjonalny)
let useFullTranslationIds = true // Bool |  (opcjonalny)
let parentId = "parentId_example" // String |  (opcjonalny)
let searchText = "searchText_example" // String |  (opcjonalny)
let hashTags = ["inner_example"] // [String] |  (opcjonalny)
let userId = "userId_example" // String |  (opcjonalny)
let customConfigStr = "customConfigStr_example" // String |  (opcjonalny)
let afterCommentId = "afterCommentId_example" // String |  (opcjonalny)
let beforeCommentId = "beforeCommentId_example" // String |  (opcjonalny)

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