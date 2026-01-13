## 參數

| 名稱 | 型別 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| page | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| skip | integer | query | 否 |  |
| asTree | boolean | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |
| contextUserId | string | query | 否 |  |
| hashTag | string | query | 否 |  |
| parentId | string | query | 否 |  |
| direction | string | query | 否 |  |

## 回應

回傳: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## 範例

[inline-code-attrs-start title = 'getComments 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為測試版本。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (選填)
let limit = 987 // Int |  (選填)
let skip = 987 // Int |  (選填)
let asTree = true // Bool |  (選填)
let skipChildren = 987 // Int |  (選填)
let limitChildren = 987 // Int |  (選填)
let maxTreeDepth = 987 // Int |  (選填)
let urlId = "urlId_example" // String |  (選填)
let userId = "userId_example" // String |  (選填)
let anonUserId = "anonUserId_example" // String |  (選填)
let contextUserId = "contextUserId_example" // String |  (選填)
let hashTag = "hashTag_example" // String |  (選填)
let parentId = "parentId_example" // String |  (選填)
let direction = SortDirections() // SortDirections |  (選填)

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

---