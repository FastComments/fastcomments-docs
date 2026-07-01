## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| page | integer | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| skip | integer | query | 아니오 |  |
| asTree | boolean | query | 아니오 |  |
| skipChildren | integer | query | 아니오 |  |
| limitChildren | integer | query | 아니오 |  |
| maxTreeDepth | integer | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| userId | string | query | 아니오 |  |
| anonUserId | string | query | 아니오 |  |
| contextUserId | string | query | 아니오 |  |
| hashTag | string | query | 아니오 |  |
| parentId | string | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| fromDate | integer | query | 아니오 |  |
| toDate | integer | query | 아니오 |  |

## Response

반환: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Example

[inline-code-attrs-start title = 'getComments 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제 발생 시, http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (옵션)
let limit = 987 // Int |  (옵션)
let skip = 987 // Int |  (옵션)
let asTree = true // Bool |  (옵션)
let skipChildren = 987 // Int |  (옵션)
let limitChildren = 987 // Int |  (옵션)
let maxTreeDepth = 987 // Int |  (옵션)
let urlId = "urlId_example" // String |  (옵션)
let userId = "userId_example" // String |  (옵션)
let anonUserId = "anonUserId_example" // String |  (옵션)
let contextUserId = "contextUserId_example" // String |  (옵션)
let hashTag = "hashTag_example" // String |  (옵션)
let parentId = "parentId_example" // String |  (옵션)
let direction = SortDirections() // SortDirections |  (옵션)
let fromDate = 987 // Int64 |  (옵션)
let toDate = 987 // Int64 |  (옵션)

DefaultAPI.getComments(tenantId: tenantId, options: DefaultAPI.GetCommentsOptions(page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction, fromDate: fromDate, toDate: toDate)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]