## Parameters

| שם | סוג | מיקום | הכרחי | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| reviewed | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Response

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Example

[inline-code-attrs-start title = 'דוגמת postSetCommentReviewStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדוגמאות בקוד שלמטה עדיין בגרסה בטא. לכל בעיה, אנא דווח באמצעות http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let reviewed = true // Bool |  (אופציונלי)
let broadcastId = "broadcastId_example" // String |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.postSetCommentReviewStatus(tenantId: tenantId, commentId: commentId, options: ModerationAPI.PostSetCommentReviewStatusOptions(reviewed: reviewed, broadcastId: broadcastId, sso: sso)) { (response, error) in
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