req
tenantId
urlId

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
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

## Risposta

Restituisce: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (opzionale)
let direction = SortDirections() // SortDirections |  (opzionale)
let sso = "sso_example" // String |  (opzionale)
let skip = 987 // Int |  (opzionale)
let skipChildren = 987 // Int |  (opzionale)
let limit = 987 // Int |  (opzionale)
let limitChildren = 987 // Int |  (opzionale)
let countChildren = true // Bool |  (opzionale)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (opzionale)
let includeConfig = true // Bool |  (opzionale)
let countAll = true // Bool |  (opzionale)
let includei10n = true // Bool |  (opzionale)
let locale = "locale_example" // String |  (opzionale)
let modules = "modules_example" // String |  (opzionale)
let isCrawler = true // Bool |  (opzionale)
let includeNotificationCount = true // Bool |  (opzionale)
let asTree = true // Bool |  (opzionale)
let maxTreeDepth = 987 // Int |  (opzionale)
let useFullTranslationIds = true // Bool |  (opzionale)
let parentId = "parentId_example" // String |  (opzionale)
let searchText = "searchText_example" // String |  (opzionale)
let hashTags = ["inner_example"] // [String] |  (opzionale)
let userId = "userId_example" // String |  (opzionale)
let customConfigStr = "customConfigStr_example" // String |  (opzionale)
let afterCommentId = "afterCommentId_example" // String |  (opzionale)
let beforeCommentId = "beforeCommentId_example" // String |  (opzionale)

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