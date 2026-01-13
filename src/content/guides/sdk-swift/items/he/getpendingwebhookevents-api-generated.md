## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | שאילתה | כן |  |
| commentId | string | שאילתה | לא |  |
| externalId | string | שאילתה | לא |  |
| eventType | string | שאילתה | לא |  |
| type | string | שאילתה | לא |  |
| domain | string | שאילתה | לא |  |
| attemptCountGT | number | שאילתה | לא |  |
| skip | number | שאילתה | לא |  |

## תגובה

מחזיר: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPendingWebhookEvents'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. אם יש בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (אופציונלי)
let externalId = "externalId_example" // String |  (אופציונלי)
let eventType = "eventType_example" // String |  (אופציונלי)
let type = "type_example" // String |  (אופציונלי)
let domain = "domain_example" // String |  (אופציונלי)
let attemptCountGT = 987 // Double |  (אופציונלי)
let skip = 987 // Double |  (אופציונלי)

DefaultAPI.getPendingWebhookEvents(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]