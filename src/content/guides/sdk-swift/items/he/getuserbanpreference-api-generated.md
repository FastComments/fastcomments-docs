## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| sso | string | query | לא |  |

## תגובה

מחזיר: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIModerateGetUserBanPreferencesResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserBanPreference'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדוגמאות הבאות של הקוד עדיין בבטא. לכל בעיה, נא לדווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.getUserBanPreference(sso: sso) { (response, error) in
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