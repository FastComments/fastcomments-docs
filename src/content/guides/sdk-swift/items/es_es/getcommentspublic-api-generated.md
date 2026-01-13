---
req
tenantId
urlId

## Parámetros

| Name | Type | Location | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |
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

## Respuesta

Devuelve: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (opcional)
let direction = SortDirections() // SortDirections |  (opcional)
let sso = "sso_example" // String |  (opcional)
let skip = 987 // Int |  (opcional)
let skipChildren = 987 // Int |  (opcional)
let limit = 987 // Int |  (opcional)
let limitChildren = 987 // Int |  (opcional)
let countChildren = true // Bool |  (opcional)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (opcional)
let includeConfig = true // Bool |  (opcional)
let countAll = true // Bool |  (opcional)
let includei10n = true // Bool |  (opcional)
let locale = "locale_example" // String |  (opcional)
let modules = "modules_example" // String |  (opcional)
let isCrawler = true // Bool |  (opcional)
let includeNotificationCount = true // Bool |  (opcional)
let asTree = true // Bool |  (opcional)
let maxTreeDepth = 987 // Int |  (opcional)
let useFullTranslationIds = true // Bool |  (opcional)
let parentId = "parentId_example" // String |  (opcional)
let searchText = "searchText_example" // String |  (opcional)
let hashTags = ["inner_example"] // [String] |  (opcional)
let userId = "userId_example" // String |  (opcional)
let customConfigStr = "customConfigStr_example" // String |  (opcional)
let afterCommentId = "afterCommentId_example" // String |  (opcional)
let beforeCommentId = "beforeCommentId_example" // String |  (opcional)

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

---