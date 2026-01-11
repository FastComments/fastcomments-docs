req
tenantId
urlId

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| page | integer | query | Нет |  |
| direction | string | query | Нет |  |
| sso | string | query | Нет |  |
| skip | integer | query | Нет |  |
| skipChildren | integer | query | Нет |  |
| limit | integer | query | Нет |  |
| limitChildren | integer | query | Нет |  |
| countChildren | boolean | query | Нет |  |
| fetchPageForCommentId | string | query | Нет |  |
| includeConfig | boolean | query | Нет |  |
| countAll | boolean | query | Нет |  |
| includei10n | boolean | query | Нет |  |
| locale | string | query | Нет |  |
| modules | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |
| includeNotificationCount | boolean | query | Нет |  |
| asTree | boolean | query | Нет |  |
| maxTreeDepth | integer | query | Нет |  |
| useFullTranslationIds | boolean | query | Нет |  |
| parentId | string | query | Нет |  |
| searchText | string | query | Нет |  |
| hashTags | array | query | Нет |  |
| userId | string | query | Нет |  |
| customConfigStr | string | query | Нет |  |
| afterCommentId | string | query | Нет |  |
| beforeCommentId | string | query | Нет |  |

## Ответ

Возвращает: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Пример

[inline-code-attrs-start title = 'getCommentsPublic Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все еще находятся в бета-версии. В случае проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (необязательно)
let direction = SortDirections() // SortDirections |  (необязательно)
let sso = "sso_example" // String |  (необязательно)
let skip = 987 // Int |  (необязательно)
let skipChildren = 987 // Int |  (необязательно)
let limit = 987 // Int |  (необязательно)
let limitChildren = 987 // Int |  (необязательно)
let countChildren = true // Bool |  (необязательно)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (необязательно)
let includeConfig = true // Bool |  (необязательно)
let countAll = true // Bool |  (необязательно)
let includei10n = true // Bool |  (необязательно)
let locale = "locale_example" // String |  (необязательно)
let modules = "modules_example" // String |  (необязательно)
let isCrawler = true // Bool |  (необязательно)
let includeNotificationCount = true // Bool |  (необязательно)
let asTree = true // Bool |  (необязательно)
let maxTreeDepth = 987 // Int |  (необязательно)
let useFullTranslationIds = true // Bool |  (необязательно)
let parentId = "parentId_example" // String |  (необязательно)
let searchText = "searchText_example" // String |  (необязательно)
let hashTags = ["inner_example"] // [String] |  (необязательно)
let userId = "userId_example" // String |  (необязательно)
let customConfigStr = "customConfigStr_example" // String |  (необязательно)
let afterCommentId = "afterCommentId_example" // String |  (необязательно)
let beforeCommentId = "beforeCommentId_example" // String |  (необязательно)

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