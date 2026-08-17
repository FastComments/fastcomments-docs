req
tenantId
urlId

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
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

## Одговор

Returns: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Пример

[inline-code-attrs-start title = 'Primer getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, молимо пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
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
let locale = "locale_example" // String |  (опционално)
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