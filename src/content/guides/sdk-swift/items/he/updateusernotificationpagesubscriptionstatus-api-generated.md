---
הפעל או השבת התראות עבור עמוד. כאשר משתמשים מנויים לעמוד, נוצרות התראות
להערות שורש חדשות, וגם

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | כן |  |
| url | string | query | כן |  |
| pageTitle | string | query | כן |  |
| subscribedOrUnsubscribed | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationPageSubscriptionStatusResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateUserNotificationPageSubscriptionStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, נא לדווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let url = "url_example" // String | 
let pageTitle = "pageTitle_example" // String | 
let subscribedOrUnsubscribed = "subscribedOrUnsubscribed_example" // String | 
let sso = "sso_example" // String |  (אופציונלי)

PublicAPI.updateUserNotificationPageSubscriptionStatus(tenantId: tenantId, urlId: urlId, url: url, pageTitle: pageTitle, subscribedOrUnsubscribed: subscribedOrUnsubscribed, sso: sso) { (response, error) in
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