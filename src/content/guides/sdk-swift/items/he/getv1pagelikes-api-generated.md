## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| sso | string | query | לא |  |
| includeUserInfo | string | query | לא | כאשר מוגדר, מחזיר את המשתמשים שאהבו את העמוד (תמונות פרופיל ושמות משתמש). רק \"public\" מתקבל, מה שמסיר פרופילים פרטיים. |

## תגובה

מחזיר: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getV1PageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | כאשר מוגדר, מחזיר את המשתמשים שאהבו את העמוד (תמונות פרופיל ושמות משתמש). רק \"public\" מתקבל, מה שמסיר פרופילים פרטיים. (optional)

PublicAPI.getV1PageLikes(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV1PageLikesOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]