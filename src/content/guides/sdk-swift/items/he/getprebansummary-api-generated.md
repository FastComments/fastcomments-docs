## פרמטרים

| Name | Type | Location | נדרש | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| includeByUserIdAndEmail | boolean | query | לא |  |
| includeByIP | boolean | query | לא |  |
| includeByEmailDomain | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`PreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PreBanSummary.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPreBanSummary'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeByUserIdAndEmail = true // Bool |  (אופציונלי)
let includeByIP = true // Bool |  (אופציונלי)
let includeByEmailDomain = true // Bool |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.getPreBanSummary(commentId: commentId, includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso) { (response, error) in
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