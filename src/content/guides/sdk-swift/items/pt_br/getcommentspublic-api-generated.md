req
tenantId
urlId

## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| page | integer | query | Não |  |
| direction | string | query | Não |  |
| sso | string | query | Não |  |
| skip | integer | query | Não |  |
| skipChildren | integer | query | Não |  |
| limit | integer | query | Não |  |
| limitChildren | integer | query | Não |  |
| countChildren | boolean | query | Não |  |
| fetchPageForCommentId | string | query | Não |  |
| includeConfig | boolean | query | Não |  |
| countAll | boolean | query | Não |  |
| includei10n | boolean | query | Não |  |
| locale | string | query | Não |  |
| modules | string | query | Não |  |
| isCrawler | boolean | query | Não |  |
| includeNotificationCount | boolean | query | Não |  |
| asTree | boolean | query | Não |  |
| maxTreeDepth | integer | query | Não |  |
| useFullTranslationIds | boolean | query | Não |  |
| parentId | string | query | Não |  |
| searchText | string | query | Não |  |
| hashTags | array | query | Não |  |
| userId | string | query | Não |  |
| customConfigStr | string | query | Não |  |
| afterCommentId | string | query | Não |  |
| beforeCommentId | string | query | Não |  |

## Resposta

Retorna: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor relate via http://github.com/OpenAPITools/openapi-generator/issues/new
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