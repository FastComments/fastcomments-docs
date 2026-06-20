## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PostRemoveCommentResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postRemoveComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. במקרה של בעיה, דווחו בבקשה דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.postRemoveComment(commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]