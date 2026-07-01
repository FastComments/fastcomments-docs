## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## תגובה

מחזיר: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדגמי קוד הבאים עדיין בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
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