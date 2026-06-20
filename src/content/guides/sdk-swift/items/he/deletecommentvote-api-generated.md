## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | נתיב | כן |  |
| commentId | string | נתיב | כן |  |
| voteId | string | נתיב | כן |  |
| urlId | string | שאילתה | כן |  |
| broadcastId | string | שאילתה | כן |  |
| editKey | string | שאילתה | לא |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteDeleteResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteCommentVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, נא לדווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voteId = "voteId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let editKey = "editKey_example" // String |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

PublicAPI.deleteCommentVote(tenantId: tenantId, commentId: commentId, voteId: voteId, urlId: urlId, broadcastId: broadcastId, editKey: editKey, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]