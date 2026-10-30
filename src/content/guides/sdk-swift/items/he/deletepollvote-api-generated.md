הסרת הצבעה. האפשרות עליה הוצבעה מחזירה את הספירה שלה.

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## Response

מחזיר: [`DeletePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeletePollVoteResponse.swift)

## Example

[inline-code-attrs-start title = 'deletePollVote דוגמה'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.deletePollVote(tenantId: tenantId, id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]