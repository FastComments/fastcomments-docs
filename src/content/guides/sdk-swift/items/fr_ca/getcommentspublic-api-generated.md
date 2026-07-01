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

Renvoie: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (facultatif)
let direction = SortDirections() // SortDirections |  (facultatif)
let sso = "sso_example" // String |  (facultatif)
let skip = 987 // Int |  (facultatif)
let skipChildren = 987 // Int |  (facultatif)
let limit = 987 // Int |  (facultatif)
let limitChildren = 987 // Int |  (facultatif)
let countChildren = true // Bool |  (facultatif)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (facultatif)
let includeConfig = true // Bool |  (facultatif)
let countAll = true // Bool |  (facultatif)
let includei10n = true // Bool |  (facultatif)
let locale = "locale_example" // String |  (facultatif)
let modules = "modules_example" // String |  (facultatif)
let isCrawler = true // Bool |  (facultatif)
let includeNotificationCount = true // Bool |  (facultatif)
let asTree = true // Bool |  (facultatif)
let maxTreeDepth = 987 // Int |  (facultatif)
let useFullTranslationIds = true // Bool |  (facultatif)
let parentId = "parentId_example" // String |  (facultatif)
let searchText = "searchText_example" // String |  (facultatif)
let hashTags = ["inner_example"] // [String] |  (facultatif)
let userId = "userId_example" // String |  (facultatif)
let customConfigStr = "customConfigStr_example" // String |  (facultatif)
let afterCommentId = "afterCommentId_example" // String |  (facultatif)
let beforeCommentId = "beforeCommentId_example" // String |  (facultatif)

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