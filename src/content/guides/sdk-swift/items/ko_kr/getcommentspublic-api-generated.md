req
tenantId
urlId

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| page | integer | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |
| skip | integer | query | 아니오 |  |
| skipChildren | integer | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| limitChildren | integer | query | 아니오 |  |
| countChildren | boolean | query | 아니오 |  |
| fetchPageForCommentId | string | query | 아니오 |  |
| includeConfig | boolean | query | 아니오 |  |
| countAll | boolean | query | 아니오 |  |
| includei10n | boolean | query | 아니오 |  |
| locale | string | query | 아니오 |  |
| modules | string | query | 아니오 |  |
| isCrawler | boolean | query | 아니오 |  |
| includeNotificationCount | boolean | query | 아니오 |  |
| asTree | boolean | query | 아니오 |  |
| maxTreeDepth | integer | query | 아니오 |  |
| useFullTranslationIds | boolean | query | 아니오 |  |
| parentId | string | query | 아니오 |  |
| searchText | string | query | 아니오 |  |
| hashTags | array | query | 아니오 |  |
| userId | string | query | 아니오 |  |
| customConfigStr | string | query | 아니오 |  |
| afterCommentId | string | query | 아니오 |  |
| beforeCommentId | string | query | 아니오 |  |

## 응답

반환: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsResponseWithPresencePublicComment.swift)

## 예제

[inline-code-attrs-start title = 'getCommentsPublic 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 여전히 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let page = 987 // Int |  (선택 사항)
let direction = SortDirections() // SortDirections |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)
let skip = 987 // Int |  (선택 사항)
let skipChildren = 987 // Int |  (선택 사항)
let limit = 987 // Int |  (선택 사항)
let limitChildren = 987 // Int |  (선택 사항)
let countChildren = true // Bool |  (선택 사항)
let fetchPageForCommentId = "fetchPageForCommentId_example" // String |  (선택 사항)
let includeConfig = true // Bool |  (선택 사항)
let countAll = true // Bool |  (선택 사항)
let includei10n = true // Bool |  (선택 사항)
let locale = "locale_example" // String |  (선택 사항)
let modules = "modules_example" // String |  (선택 사항)
let isCrawler = true // Bool |  (선택 사항)
let includeNotificationCount = true // Bool |  (선택 사항)
let asTree = true // Bool |  (선택 사항)
let maxTreeDepth = 987 // Int |  (선택 사항)
let useFullTranslationIds = true // Bool |  (선택 사항)
let parentId = "parentId_example" // String |  (선택 사항)
let searchText = "searchText_example" // String |  (선택 사항)
let hashTags = ["inner_example"] // [String] |  (선택 사항)
let userId = "userId_example" // String |  (선택 사항)
let customConfigStr = "customConfigStr_example" // String |  (선택 사항)
let afterCommentId = "afterCommentId_example" // String |  (선택 사항)
let beforeCommentId = "beforeCommentId_example" // String |  (선택 사항)

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