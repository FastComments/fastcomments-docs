## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| userId | string | query | לא |  |
| trustFactor | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-setTrustFactor'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמות הקוד הבאות עדיין בבטא. במקרה של בעיה, דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (אופציונלי)
let trustFactor = "trustFactor_example" // String |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.setTrustFactor(userId: userId, trustFactor: trustFactor, sso: sso) { (response, error) in
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