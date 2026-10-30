---
צירוף סקר לתגובה, או הגדרת המצב המלא של הסקר שכבר קיים.

האפשרויות מתואמות לפי מזהה: אפשרות שנשלחת עם המזהה של אפשרות קיימת שומרת את הקולות שלה (ולקחת את התווית והמיקום החדשים), אפשרות שנשלחת ללא מזהה מתווספת, והאפשרויות הקיימות שהושמטו מהרשימה מוסרות יחד עם הקולות שהושמעו עליהן.

שמירת אפס מזהי אפשרויות קיימות בסקר שיש לו קולות מוחקת את כולן, ולכן יש צורך ב‑replaceVotes=true.

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| replaceVotes | boolean | query | No |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Example

[inline-code-attrs-start title = 'putPoll דוגמה'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (optional)

DefaultAPI.putPoll(tenantId: tenantId, commentId: commentId, commentPollPutInput: commentPollPutInput, replaceVotes: replaceVotes) { (response, error) in
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