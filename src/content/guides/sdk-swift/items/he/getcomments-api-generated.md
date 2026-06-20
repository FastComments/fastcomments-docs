## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| page | integer | query | לא |  |
| limit | integer | query | לא |  |
| skip | integer | query | לא |  |
| asTree | boolean | query | לא |  |
| skipChildren | integer | query | לא |  |
| limitChildren | integer | query | לא |  |
| maxTreeDepth | integer | query | לא |  |
| urlId | string | query | לא |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |
| contextUserId | string | query | לא |  |
| hashTag | string | query | לא |  |
| parentId | string | query | לא |  |
| direction | string | query | לא |  |
| fromDate | integer | query | לא |  |
| toDate | integer | query | לא |  |

## תגובה

מחזיר: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (אופציונלי)
let limit = 987 // Int |  (אופציונלי)
let skip = 987 // Int |  (אופציונלי)
let asTree = true // Bool |  (אופציונלי)
let skipChildren = 987 // Int |  (אופציונלי)
let limitChildren = 987 // Int |  (אופציונלי)
let maxTreeDepth = 987 // Int |  (אופציונלי)
let urlId = "urlId_example" // String |  (אופציונלי)
let userId = "userId_example" // String |  (אופציונלי)
let anonUserId = "anonUserId_example" // String |  (אופציונלי)
let contextUserId = "contextUserId_example" // String |  (אופציונלי)
let hashTag = "hashTag_example" // String |  (אופציונלי)
let parentId = "parentId_example" // String |  (אופציונלי)
let direction = SortDirections() // SortDirections |  (אופציונלי)
let fromDate = 987 // Int64 |  (אופציונלי)
let toDate = 987 // Int64 |  (אופציונלי)

DefaultAPI.getComments(tenantId: tenantId, page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction, fromDate: fromDate, toDate: toDate) { (response, error) in
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