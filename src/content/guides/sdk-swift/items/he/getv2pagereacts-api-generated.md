## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | כאשר מוגדר, מחזיר את המשתמשים עבור כל מזהה תגובה (תמונות פרופיל ושמות משתמש). רק "public" מתקבל, מה שמסיר פרופילים פרטיים. |

## תגובה

מחזיר: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getV2PageReacts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (אופציונלי)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | כאשר מוגדר, מחזיר את המשתמשים עבור כל מזהה תגובה (תמונות פרופיל ושמות משתמש). רק \"public\" מתקבל, מה שמסיר פרופילים פרטיים. (אופציונלי)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]