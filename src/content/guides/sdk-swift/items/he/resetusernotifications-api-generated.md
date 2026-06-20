## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| afterId | string | query | לא |  |
| afterCreatedAt | integer | query | לא |  |
| unreadOnly | boolean | query | לא |  |
| dmOnly | boolean | query | לא |  |
| noDm | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ResetUserNotificationsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-resetUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לגבי כל בעיה, דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (אופציונלי)
let afterCreatedAt = 987 // Int64 |  (אופציונלי)
let unreadOnly = true // Bool |  (אופציונלי)
let dmOnly = true // Bool |  (אופציונלי)
let noDm = true // Bool |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

PublicAPI.resetUserNotifications(tenantId: tenantId, afterId: afterId, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]