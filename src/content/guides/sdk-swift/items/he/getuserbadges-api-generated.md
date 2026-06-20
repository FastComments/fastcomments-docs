## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| badgeId | string | query | לא |  |
| type | number | query | לא |  |
| displayedOnComments | boolean | query | לא |  |
| limit | number | query | לא |  |
| skip | number | query | לא |  |

## תשובה

מחזיר: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. עבור כל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (אופציונלי)
let badgeId = "badgeId_example" // String |  (אופציונלי)
let type = 987 // Double |  (אופציונלי)
let displayedOnComments = true // Bool |  (אופציונלי)
let limit = 987 // Double |  (אופציונלי)
let skip = 987 // Double |  (אופציונלי)

DefaultAPI.getUserBadges(tenantId: tenantId, userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]