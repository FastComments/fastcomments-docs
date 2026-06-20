## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| state | number | query | לא |  |
| skip | number | query | לא |  |
| limit | number | query | לא |  |

## תגובה

מחזיר: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicketsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getTickets'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (אופציונלי)
let state = 987 // Double |  (אופציונלי)
let skip = 987 // Double |  (אופציונלי)
let limit = 987 // Double |  (אופציונלי)

DefaultAPI.getTickets(tenantId: tenantId, userId: userId, state: state, skip: skip, limit: limit) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]