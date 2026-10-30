---
רשום הצבעה בסקר, או העבר הצבעה קיימת לאופציה אחרת. לכל מצביע יש לכל היותר הצבעה אחת לכל סקר, ולכן קריאה חוזרת לפעולה עבור אותו מצביע מעבירה את הצביעתו במקום להוסיף הצבעה חדשה.

זה מציית להגדרות הסקר של האתר: אם ההצבעה מוגדרת למשתמשים מחוברים בלבד, הצבעה עם רק anonUserId תידחה, והצבעות אנונימיות מוגבלות בקצב לפי IP לכל סקר.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Example

[inline-code-attrs-start title = 'דוגמת createPollVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createPollVoteBody = CreatePollVoteBody(commentId: "commentId_example", optionId: "optionId_example", userId: "userId_example", anonUserId: "anonUserId_example", ip: "ip_example") // CreatePollVoteBody | 

DefaultAPI.createPollVote(tenantId: tenantId, createPollVoteBody: createPollVoteBody) { (response, error) in
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