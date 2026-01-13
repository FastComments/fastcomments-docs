захтев
tenantId
urlId

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| page | integer | query | Не |  |
| direction | string | query | Не |  |
| sso | string | query | Не |  |
| skip | integer | query | Не |  |
| skipChildren | integer | query | Не |  |
| limit | integer | query | Не |  |
| limitChildren | integer | query | Не |  |
| countChildren | boolean | query | Не |  |
| fetchPageForCommentId | string | query | Не |  |
| includeConfig | boolean | query | Не |  |
| countAll | boolean | query | Не |  |
| includei10n | boolean | query | Не |  |
| locale | string | query | Не |  |
| modules | string | query | Не |  |
| isCrawler | boolean | query | Не |  |
| includeNotificationCount | boolean | query | Не |  |
| asTree | boolean | query | Не |  |
| maxTreeDepth | integer | query | Не |  |
| useFullTranslationIds | boolean | query | Не |  |
| parentId | string | query | Не |  |
| searchText | string | query | Не |  |
| hashTags | array | query | Не |  |
| userId | string | query | Не |  |
| customConfigStr | string | query | Не |  |
| afterCommentId | string | query | Не |  |
| beforeCommentId | string | query | Не |  |

## Одговор

Враћа: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су и даље у бета фази. За било који проблем, пријавите га путем http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (опционо)
let direction = SortDirections() // SortDirections |  (опционо)
let sso = "sso_example" // String |  (опционо)
let skip = 987 // Int |  (опционо)
let skipChildren = 987 // Int |  (опционо)
let limit = 987 // Int |  (опционо)
let limitChildren = 987 // Int |  (опционо)
let countChildren = true // Bool |  (опционо)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (опционо)
let includeConfig = true // Bool |  (опционо)
let countAll = true // Bool |  (опционо)
let includei10n = true // Bool |  (опционо)
let locale = "locale_example" // String |  (опционо)
let modules = "modules_example" // String |  (опционо)
let isCrawler = true // Bool |  (опционо)
let includeNotificationCount = true // Bool |  (опционо)
let asTree = true // Bool |  (опционо)
let maxTreeDepth = 987 // Int |  (опционо)
let useFullTranslationIds = true // Bool |  (опционо)
let parentId = "parentId_example" // String |  (опционо)
let searchText = "searchText_example" // String |  (опционо)
let hashTags = ["inner_example"] // [String] |  (опционо)
let userId = "userId_example" // String |  (опционо)
let customConfigStr = "customConfigStr_example" // String |  (опционо)
let afterCommentId = "afterCommentId_example" // String |  (опционо)
let beforeCommentId = "beforeCommentId_example" // String |  (опционо)

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