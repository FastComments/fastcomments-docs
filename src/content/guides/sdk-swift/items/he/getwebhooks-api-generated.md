רשימת ה-webhooks המוגדרים עבור ה-tenant, כולל שורות מנוהלות בלוח המחוונים ומנויים דרך ה-API.

## Parameters

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| event | string | query | לא |  |
| domain | string | query | לא |  |
| source | string | query | לא |  |
| skip | number | query | לא |  |

## Response

מחזיר: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (אופציונלי)
let domain = "domain_example" // String |  (אופציונלי)
let source = WebhookSource() // WebhookSource |  (אופציונלי)
let skip = 987 // Double |  (אופציונלי)

DefaultAPI.getWebhooks(tenantId: tenantId, options: DefaultAPI.GetWebhooksOptions(event: event, domain: domain, source: source, skip: skip)) { (response, error) in
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