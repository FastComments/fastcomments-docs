## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | משמש לקביעת האם הדף הנוכחי מנוי. |
| pageSize | integer | query | No |  |
| afterId | string | query | No |  |
| includeContext | boolean | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| includeTranslations | boolean | query | No |  |
| includeTenantNotifications | boolean | query | No |  |
| sno | string | query | No |  |

## תגובה

Returns: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדגמות הקוד הבאות עדיין בגרסת בטא. עבור כל בעיה, אנא דווח באמצעות http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | משמש לקביעת האם הדף הנוכחי מנוי. (אופציונלי)
let pageSize = 987 // Int |  (אופציונלי)
let afterId = "afterId_example" // String |  (אופציונלי)
let includeContext = true // Bool |  (אופציונלי)
let afterCreatedAt = 987 // Int64 |  (אופציונלי)
let unreadOnly = true // Bool |  (אופציונלי)
let dmOnly = true // Bool |  (אופציונלי)
let noDm = true // Bool |  (אופציונלי)
let includeTranslations = true // Bool |  (אופציונלי)
let includeTenantNotifications = true // Bool |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

PublicAPI.getUserNotifications(tenantId: tenantId, options: PublicAPI.GetUserNotificationsOptions(urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]