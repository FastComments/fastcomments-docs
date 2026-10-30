req
tenantId
urlId

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
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
| questionId | string | query | No |  |
| questionValue | integer | query | No |  |

## Ответ

Возвращает: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
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
let questionId = "questionId_example" // String |  (необязательно)
let questionValue = 987 // Int |  (необязательно)

PublicAPI.getCommentsPublic(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetCommentsPublicOptions(page: page, direction: direction, sso: sso, skip: skip, skipChildren: skipChildren, limit: limit, limitChildren: limitChildren, countChildren: countChildren, fetchPageForCommentId: fetchPageForCommentId, includeConfig: includeConfig, countAll: countAll, includei10n: includei10n, locale: locale, modules: modules, isCrawler: isCrawler, includeNotificationCount: includeNotificationCount, asTree: asTree, maxTreeDepth: maxTreeDepth, useFullTranslationIds: useFullTranslationIds, parentId: parentId, searchText: searchText, hashTags: hashTags, userId: userId, customConfigStr: customConfigStr, afterCommentId: afterCommentId, beforeCommentId: beforeCommentId, questionId: questionId, questionValue: questionValue)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]