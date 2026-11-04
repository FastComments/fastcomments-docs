ספירות תגובות מותאמות אישית של react עבור דף, אופציונלית עם המשתמשים שהגיבו.

## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | כן |  |
| includeUserInfo | string | query | לא | "public" מדלג על משתמשים עם פרופילים פרטיים, "include-private" כולל אותם. |

## תגובה

מחזיר: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageReactsAPIResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPageReacts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" מדלג על משתמשים עם פרופילים פרטיים, \"include-private\" כולל אותם. (optional)

DefaultAPI.getPageReacts(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]