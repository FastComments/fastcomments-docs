req
tenantId
urlId

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
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

## Svar

Returnerer: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Eksempel

[inline-code-attrs-start title = 'getCommentsPublic Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (valgfri)
let direction = SortDirections() // SortDirections |  (valgfri)
let sso = "sso_example" // String |  (valgfri)
let skip = 987 // Int |  (valgfri)
let skipChildren = 987 // Int |  (valgfri)
let limit = 987 // Int |  (valgfri)
let limitChildren = 987 // Int |  (valgfri)
let countChildren = true // Bool |  (valgfri)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (valgfri)
let includeConfig = true // Bool |  (valgfri)
let countAll = true // Bool |  (valgfri)
let includei10n = true // Bool |  (valgfri)
let locale = "locale_example" // String |  (valgfri)
let modules = "modules_example" // String |  (valgfri)
let isCrawler = true // Bool |  (valgfri)
let includeNotificationCount = true // Bool |  (valgfri)
let asTree = true // Bool |  (valgfri)
let maxTreeDepth = 987 // Int |  (valgfri)
let useFullTranslationIds = true // Bool |  (valgfri)
let parentId = "parentId_example" // String |  (valgfri)
let searchText = "searchText_example" // String |  (valgfri)
let hashTags = ["inner_example"] // [String] |  (valgfri)
let userId = "userId_example" // String |  (valgfri)
let customConfigStr = "customConfigStr_example" // String |  (valgfri)
let afterCommentId = "afterCommentId_example" // String |  (valgfri)
let beforeCommentId = "beforeCommentId_example" // String |  (valgfri)

PublicAPI.getCommentsPublic(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetCommentsPublicOptions(page: page, direction: direction, sso: sss, skip: skip, skipChildren: skipChildren, limit: limit, limitChildren: limitChildren, countChildren: countChildren, fetchPageForCommentId: fetchPageForCommentId, includeConfig: includeConfig, countAll: countAll, includei10n: includei10n, locale: locale, modules: modules, isCrawler: isCrawler, includeNotificationCount: includeNotificationCount, asTree: asTree, maxTreeDepth: maxTreeDepth, useFullTranslationIds: useFullTranslationIds, parentId: parentId, searchText: searchText, hashTags: hashTags, userId: userId, customConfigStr: customConfigStr, afterCommentId: afterCommentId, beforeCommentId: beforeCommentId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---