## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBannedUsersCountResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCounts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדוגמאות הבאות של הקוד עדיין בבטא. עבור כל בעיה, נא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.getCounts(sso: sso) { (response, error) in
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