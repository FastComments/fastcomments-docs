## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| page | integer | query | 아니요 |  |
| limit | integer | query | 아니요 |  |
| skip | integer | query | 아니요 |  |
| asTree | boolean | query | 아니요 |  |
| skipChildren | integer | query | 아니요 |  |
| limitChildren | integer | query | 아니요 |  |
| maxTreeDepth | integer | query | 아니요 |  |
| urlId | string | query | 아니요 |  |
| userId | string | query | 아니요 |  |
| anonUserId | string | query | 아니요 |  |
| contextUserId | string | query | 아니요 |  |
| hashTag | string | query | 아니요 |  |
| parentId | string | query | 아니요 |  |
| direction | string | query | 아니요 |  |

## 응답

반환: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## 예제

[inline-code-attrs-start title = 'getComments 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (선택 사항)
let limit = 987 // Int |  (선택 사항)
let skip = 987 // Int |  (선택 사항)
let asTree = true // Bool |  (선택 사항)
let skipChildren = 987 // Int |  (선택 사항)
let limitChildren = 987 // Int |  (선택 사항)
let maxTreeDepth = 987 // Int |  (선택 사항)
let urlId = "urlId_example" // String |  (선택 사항)
let userId = "userId_example" // String |  (선택 사항)
let anonUserId = "anonUserId_example" // String |  (선택 사항)
let contextUserId = "contextUserId_example" // String |  (선택 사항)
let hashTag = "hashTag_example" // String |  (선택 사항)
let parentId = "parentId_example" // String |  (선택 사항)
let direction = SortDirections() // SortDirections |  (선택 사항)

DefaultAPI.getComments(tenantId: tenantId, page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]