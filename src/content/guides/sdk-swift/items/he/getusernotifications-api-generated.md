## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| pageSize | integer | query | לא |  |
| afterId | string | query | לא |  |
| includeContext | boolean | query | לא |  |
| afterCreatedAt | integer | query | לא |  |
| unreadOnly | boolean | query | לא |  |
| dmOnly | boolean | query | לא |  |
| noDm | boolean | query | לא |  |
| includeTranslations | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserNotifications200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'getUserNotifications דוגמה'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד שלהלן עדיין בבטא. עבור כל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let pageSize = 987 // Int |  (אופציונלי)
let afterId = "afterId_example" // String |  (אופציונלי)
let includeContext = true // Bool |  (אופציונלי)
let afterCreatedAt = 987 // Int64 |  (אופציונלי)
let unreadOnly = true // Bool |  (אופציונלי)
let dmOnly = true // Bool |  (אופציונלי)
let noDm = true // Bool |  (אופציונלי)
let includeTranslations = true // Bool |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

PublicAPI.getUserNotifications(tenantId: tenantId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, sso: sso) { (response, error) in
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