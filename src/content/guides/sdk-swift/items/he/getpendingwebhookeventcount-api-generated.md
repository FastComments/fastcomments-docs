## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | query | לא |  |
| externalId | string | query | לא |  |
| eventType | string | query | לא |  |
| type | string | query | לא |  |
| domain | string | query | לא |  |
| attemptCountGT | number | query | לא |  |

## תגובה

מחזיר: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCount200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של getPendingWebhookEventCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדוגמאות הבאות של הקוד עדיין בבטא. לכל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (אופציונלי)
let externalId = "externalId_example" // String |  (אופציונלי)
let eventType = "eventType_example" // String |  (אופציונלי)
let type = "type_example" // String |  (אופציונלי)
let domain = "domain_example" // String |  (אופציונלי)
let attemptCountGT = 987 // Double |  (אופציונלי)

DefaultAPI.getPendingWebhookEventCount(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT) { (response, error) in
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