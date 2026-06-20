req
tenantId
urlId
userIdWS

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| userIdWS | string | query | כן |  |
| startTime | integer | query | כן |  |
| endTime | integer | query | לא |  |

## תגובה

מחזיר: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEventLogResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getGlobalEventLog'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד שלהלן עדיין בבטא. עבור כל בעיה, דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let userIdWS = "userIdWS_example" // String | 
let startTime = 987 // Int64 | 
let endTime = 987 // Int64 |  (אופציונלי)

PublicAPI.getGlobalEventLog(tenantId: tenantId, urlId: urlId, userIdWS: userIdWS, startTime: startTime, endTime: endTime) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]