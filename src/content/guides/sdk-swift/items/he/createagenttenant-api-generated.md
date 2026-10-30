---
יוצר חשבון ניסיון חדש עבור סוכן AI ללא הרשמה של אדם. אין צורך במפתח API כדי לקרוא לזה.

התגובה מכילה את מזהה השוכר, מפתח API הפועל מיידית נגד ה-REST API ושרת ה-MCP, וכן כתובת URL לתביעת החשבון. יש לתת את כתובת ה-URL לתביעה לאדם שאתה עובד עבורו: פתיחת הקישור בזמן שאתה מחובר ל-FastComments מחברת את החשבון אליו. חשבונות שלא נדרשו, והמפתחות שלהם, נמחקים 72 שעות לאחר היצירה. עד לתביעה, לחשבון יש את מגבלות הניסיון הסטנדרטיות.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'דוגמת createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let createAgentTenantBody = CreateAgentTenantBody(name: "name_example", domains: ["domains_example"], agent: "agent_example") // CreateAgentTenantBody | 

DefaultAPI.createAgentTenant(createAgentTenantBody: createAgentTenantBody) { (response, error) in
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