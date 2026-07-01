req
tenantId
urlId

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
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

## Réponse

Retourne : [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les échantillons de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
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