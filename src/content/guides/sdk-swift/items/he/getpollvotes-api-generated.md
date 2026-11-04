הקולות הבודדים מאחורי ספירות הסקר, מהישן לחדש.

סקר שייך לתגובה, ולכן קולות נקראים תמיד סקר אחד בכל פעם - commentId נדרש. זה
מבטיח שכל שאילתה משתמשת באינדקסים שכבר קיימים באוסף.

מכבד את פרטיות הסקר: לא ניתן לקרוא את הקולות של סקר אנונימי (poll-anonymous), כאן או לפי מזהה.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | query | כן |  |
| voterId | string | query | לא |  |
| optionId | string | query | לא |  |
| skip | integer | query | לא |  |

## תגובה

מחזיר: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (optional)
let optionId = "optionId_example" // String |  (optional)
let skip = 987 // Int |  (optional)

DefaultAPI.getPollVotes(tenantId: tenantId, commentId: commentId, options: DefaultAPI.GetPollVotesOptions(voterId: voterId, optionId: optionId, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]