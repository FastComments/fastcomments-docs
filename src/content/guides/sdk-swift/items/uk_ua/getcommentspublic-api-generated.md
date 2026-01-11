req
tenantId
urlId

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| page | integer | query | Ні |  |
| direction | string | query | Ні |  |
| sso | string | query | Ні |  |
| skip | integer | query | Ні |  |
| skipChildren | integer | query | Ні |  |
| limit | integer | query | Ні |  |
| limitChildren | integer | query | Ні |  |
| countChildren | boolean | query | Ні |  |
| fetchPageForCommentId | string | query | Ні |  |
| includeConfig | boolean | query | Ні |  |
| countAll | boolean | query | Ні |  |
| includei10n | boolean | query | Ні |  |
| locale | string | query | Ні |  |
| modules | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |
| includeNotificationCount | boolean | query | Ні |  |
| asTree | boolean | query | Ні |  |
| maxTreeDepth | integer | query | Ні |  |
| useFullTranslationIds | boolean | query | Ні |  |
| parentId | string | query | Ні |  |
| searchText | string | query | Ні |  |
| hashTags | array | query | Ні |  |
| userId | string | query | Ні |  |
| customConfigStr | string | query | Ні |  |
| afterCommentId | string | query | Ні |  |
| beforeCommentId | string | query | Ні |  |

## Відповідь

Повертає: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. У разі будь-яких проблем, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (необов'язково)
let direction = SortDirections() // SortDirections |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)
let skip = 987 // Int |  (необов'язково)
let skipChildren = 987 // Int |  (необов'язково)
let limit = 987 // Int |  (необов'язково)
let limitChildren = 987 // Int |  (необов'язково)
let countChildren = true // Bool |  (необов'язково)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (необов'язково)
let includeConfig = true // Bool |  (необов'язково)
let countAll = true // Bool |  (необов'язково)
let includei10n = true // Bool |  (необов'язково)
let locale = "locale_example" // String |  (необов'язково)
let modules = "modules_example" // String |  (необов'язково)
let isCrawler = true // Bool |  (необов'язково)
let includeNotificationCount = true // Bool |  (необов'язково)
let asTree = true // Bool |  (необов'язково)
let maxTreeDepth = 987 // Int |  (необов'язково)
let useFullTranslationIds = true // Bool |  (необов'язково)
let parentId = "parentId_example" // String |  (необов'язково)
let searchText = "searchText_example" // String |  (необов'язково)
let hashTags = ["inner_example"] // [String] |  (необов'язково)
let userId = "userId_example" // String |  (необов'язково)
let customConfigStr = "customConfigStr_example" // String |  (необов'язково)
let afterCommentId = "afterCommentId_example" // String |  (необов'язково)
let beforeCommentId = "beforeCommentId_example" // String |  (необов'язково)

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