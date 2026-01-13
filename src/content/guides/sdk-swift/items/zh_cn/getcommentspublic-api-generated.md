req
tenantId
urlId

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| page | integer | query | 否 |  |
| direction | string | query | 否 |  |
| sso | string | query | 否 |  |
| skip | integer | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| countChildren | boolean | query | 否 |  |
| fetchPageForCommentId | string | query | 否 |  |
| includeConfig | boolean | query | 否 |  |
| countAll | boolean | query | 否 |  |
| includei10n | boolean | query | 否 |  |
| locale | string | query | 否 |  |
| modules | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeNotificationCount | boolean | query | 否 |  |
| asTree | boolean | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |
| parentId | string | query | 否 |  |
| searchText | string | query | 否 |  |
| hashTags | array | query | 否 |  |
| userId | string | query | 否 |  |
| customConfigStr | string | query | 否 |  |
| afterCommentId | string | query | 否 |  |
| beforeCommentId | string | query | 否 |  |

## 响应

返回: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsPublic200Response.swift)

## 示例

[inline-code-attrs-start title = 'getCommentsPublic 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。若有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  （可选）
let direction = SortDirections() // SortDirections |  （可选）
let sso = "sso_example" // String |  （可选）
let skip = 987 // Int |  （可选）
let skipChildren = 987 // Int |  （可选）
let limit = 987 // Int |  （可选）
let limitChildren = 987 // Int |  （可选）
let countChildren = true // Bool |  （可选）
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  （可选）
let includeConfig = true // Bool |  （可选）
let countAll = true // Bool |  （可选）
let includei10n = true // Bool |  （可选）
let locale = "locale_example" // String |  （可选）
let modules = "modules_example" // String |  （可选）
let isCrawler = true // Bool |  （可选）
let includeNotificationCount = true // Bool |  （可选）
let asTree = true // Bool |  （可选）
let maxTreeDepth = 987 // Int |  （可选）
let useFullTranslationIds = true // Bool |  （可选）
let parentId = "parentId_example" // String |  （可选）
let searchText = "searchText_example" // String |  （可选）
let hashTags = ["inner_example"] // [String] |  （可选）
let userId = "userId_example" // String |  （可选）
let customConfigStr = "customConfigStr_example" // String |  （可选）
let afterCommentId = "afterCommentId_example" // String |  （可选）
let beforeCommentId = "beforeCommentId_example" // String |  （可选）

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