req
tenantId
urlId

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
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

[inline-code-attrs-start title = 'getCommentsPublic Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још увијек у бета верзији. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (опционално)
let direction = SortDirections() // SortDirections |  (опционално)
let sso = "sso_example" // String |  (опционално)
let skip = 987 // Int |  (опционално)
let skipChildren = 987 // Int |  (опционално)
let limit = 987 // Int |  (опционално)
let limitChildren = 987 // Int |  (опционално)
let countChildren = true // Bool |  (опционално)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (опционално)
let includeConfig = true // Bool |  (опционално)
let countAll = true // Bool |  (опционално)
let includei10n = true // Bool |  (опционално)
let locale = "locale_example" // String |  (опicionално)
let modules = "modules_example" // String |  (опционално)
let isCrawler = true // Bool |  (опционално)
let includeNotificationCount = true // Bool |  (опционално)
let asTree = true // Bool |  (опционално)
let maxTreeDepth = 987 // Int |  (опционално)
let useFullTranslationIds = true // Bool |  (опционално)
let parentId = "parentId_example" // String |  (опционално)
let searchText = "searchText_example" // String |  (опционално)
let hashTags = ["inner_example"] // [String] |  (опционално)
let userId = "userId_example" // String |  (опционално)
let customConfigStr = "customConfigStr_example" // String |  (опционално)
let afterCommentId = "afterCommentId_example" // String |  (опционално)
let beforeCommentId = "beforeCommentId_example" // String |  (опционално)

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