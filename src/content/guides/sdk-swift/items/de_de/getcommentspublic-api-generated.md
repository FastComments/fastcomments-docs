req
tenantId
urlId

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nein |  |
| direction | string | query | Nein |  |
| sso | string | query | Nein |  |
| skip | integer | query | Nein |  |
| skipChildren | integer | query | Nein |  |
| limit | integer | query | Nein |  |
| limitChildren | integer | query | Nein |  |
| countChildren | boolean | query | Nein |  |
| fetchPageForCommentId | string | query | Nein |  |
| includeConfig | boolean | query | Nein |  |
| countAll | boolean | query | Nein |  |
| includei10n | boolean | query | Nein |  |
| locale | string | query | Nein |  |
| modules | string | query | Nein |  |
| isCrawler | boolean | query | Nein |  |
| includeNotificationCount | boolean | query | Nein |  |
| asTree | boolean | query | Nein |  |
| maxTreeDepth | integer | query | Nein |  |
| useFullTranslationIds | boolean | query | Nein |  |
| parentId | string | query | Nein |  |
| searchText | string | query | Nein |  |
| hashTags | array | query | Nein |  |
| userId | string | query | Nein |  |
| customConfigStr | string | query | Nein |  |
| afterCommentId | string | query | Nein |  |
| beforeCommentId | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta-Phase. Bei Problemen melden Sie sich bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
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