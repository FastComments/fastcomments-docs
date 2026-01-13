req
tenantId
urlId

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nej |  |
| direction | string | query | Nej |  |
| sso | string | query | Nej |  |
| skip | integer | query | Nej |  |
| skipChildren | integer | query | Nej |  |
| limit | integer | query | Nej |  |
| limitChildren | integer | query | Nej |  |
| countChildren | boolean | query | Nej |  |
| fetchPageForCommentId | string | query | Nej |  |
| includeConfig | boolean | query | Nej |  |
| countAll | boolean | query | Nej |  |
| includei10n | boolean | query | Nej |  |
| locale | string | query | Nej |  |
| modules | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |
| includeNotificationCount | boolean | query | Nej |  |
| asTree | boolean | query | Nej |  |
| maxTreeDepth | integer | query | Nej |  |
| useFullTranslationIds | boolean | query | Nej |  |
| parentId | string | query | Nej |  |
| searchText | string | query | Nej |  |
| hashTags | array | query | Nej |  |
| userId | string | query | Nej |  |
| customConfigStr | string | query | Nej |  |
| afterCommentId | string | query | Nej |  |
| beforeCommentId | string | query | Nej |  |

## Respons

Returnerer: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'getCommentsPublic Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. Hvis der opstår et problem, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
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