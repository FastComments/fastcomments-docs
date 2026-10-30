ערוך סקר במקום, תוך שמירה על הספירות שלו: שנה את השאלה, שנה תווית של אפשרות, סגור או פתח אותו מחדש, או שנה מי יכול לראות את המצביעים. אפשרויות מזוהות לפי מזהה - כדי להוסיף, להסיר או לסדר מחדש אותן, השתמש ב‑PUT על הרשימה המלאה.

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | שאילתה | כן |  |
| commentId | string | נתיב | כן |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Example

[inline-code-attrs-start title = 'patchPoll דוגמה'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPatch = CommentPollPatch(question: "question_example", options: [CommentPollPatch_options_inner(label: "label_example", id: "id_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPatch | 

DefaultAPI.patchPoll(tenantId: tenantId, commentId: commentId, commentPollPatch: commentPollPatch) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]