req
tenantId
urlId

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| page | integer | query | Non |  |
| direction | string | query | Non |  |
| sso | string | query | Non |  |
| skip | integer | query | Non |  |
| skipChildren | integer | query | Non |  |
| limit | integer | query | Non |  |
| limitChildren | integer | query | Non |  |
| countChildren | boolean | query | Non |  |
| fetchPageForCommentId | string | query | Non |  |
| includeConfig | boolean | query | Non |  |
| countAll | boolean | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| modules | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeNotificationCount | boolean | query | Non |  |
| asTree | boolean | query | Non |  |
| maxTreeDepth | integer | query | Non |  |
| useFullTranslationIds | boolean | query | Non |  |
| parentId | string | query | Non |  |
| searchText | string | query | Non |  |
| hashTags | array | query | Non |  |
| userId | string | query | Non |  |
| customConfigStr | string | query | Non |  |
| afterCommentId | string | query | Non |  |
| beforeCommentId | string | query | Non |  |

## Réponse

Renvoie: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (optionnel)
let direction = SortDirections() // SortDirections |  (optionnel)
let sso = "sso_example" // String |  (optionnel)
let skip = 987 // Int |  (optionnel)
let skipChildren = 987 // Int |  (optionnel)
let limit = 987 // Int |  (optionnel)
let limitChildren = 987 // Int |  (optionnel)
let countChildren = true // Bool |  (optionnel)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (optionnel)
let includeConfig = true // Bool |  (optionnel)
let countAll = true // Bool |  (optionnel)
let includei10n = true // Bool |  (optionnel)
let locale = "locale_example" // String |  (optionnel)
let modules = "modules_example" // String |  (optionnel)
let isCrawler = true // Bool |  (optionnel)
let includeNotificationCount = true // Bool |  (optionnel)
let asTree = true // Bool |  (optionnel)
let maxTreeDepth = 987 // Int |  (optionnel)
let useFullTranslationIds = true // Bool |  (optionnel)
let parentId = "parentId_example" // String |  (optionnel)
let searchText = "searchText_example" // String |  (optionnel)
let hashTags = ["inner_example"] // [String] |  (optionnel)
let userId = "userId_example" // String |  (optionnel)
let customConfigStr = "customConfigStr_example" // String |  (optionnel)
let afterCommentId = "afterCommentId_example" // String |  (optionnel)
let beforeCommentId = "beforeCommentId_example" // String |  (optionnel)

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