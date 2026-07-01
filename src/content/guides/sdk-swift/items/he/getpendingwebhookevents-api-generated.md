## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | query | לא |  |
| externalId | string | query | לא |  |
| eventType | string | query | לא |  |
| type | string | query | לא |  |
| domain | string | query | לא |  |
| attemptCountGT | number | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getPendingWebhookEvents'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדגימות בקוד למטה עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (אופציונלי)
let externalId = "externalId_example" // String |  (אופציונלי)
let eventType = "eventType_example" // String |  (אופציונלי)
let type = "type_example" // String |  (אופציונלי)
let domain = "domain_example" // String |  (אופציונלי)
let attemptCountGT = 987 // Double |  (אופציונלי)
let skip = 987 // Double |  (אופציונלי)

DefaultAPI.getPendingWebhookEvents(tenantId: tenantId, options: DefaultAPI.GetPendingWebhookEventsOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip)) { (response, error) in
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