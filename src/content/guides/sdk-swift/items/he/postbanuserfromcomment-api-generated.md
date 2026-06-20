## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| banEmail | boolean | query | לא |  |
| banEmailDomain | boolean | query | לא |  |
| banIP | boolean | query | לא |  |
| deleteAllUsersComments | boolean | query | לא |  |
| bannedUntil | string | query | לא |  |
| isShadowBan | boolean | query | לא |  |
| updateId | string | query | לא |  |
| banReason | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postBanUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד שלהלן עדיין בגרסת בטא. לכל בעיה, דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (אופציונלי)
let banEmailDomain = true // Bool |  (אופציונלי)
let banIP = true // Bool |  (אופציונלי)
let deleteAllUsersComments = true // Bool |  (אופציונלי)
let bannedUntil = "bannedUntil_example" // String |  (אופציונלי)
let isShadowBan = true // Bool |  (אופציונלי)
let updateId = "updateId_example" // String |  (אופציונלי)
let banReason = "banReason_example" // String |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.postBanUserFromComment(commentId: commentId, banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso) { (response, error) in
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