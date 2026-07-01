Past commenters on the page who are NOT currently online. Sorted by displayName.  
המתגדים הקודמים בדף שאינם מחוברים כרגע. ממוין לפי displayName.

Use this after exhausting /users/online to render a "Members" section.  
השתמש בזה לאחר שחילצת את /users/online כדי להציג את חלקת "Members".

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
דפדוף בעזרת курсור על commenterName: השרת מעביר את האינדקס החלקי {tenantId, urlId, commenterName} מ‑afterName קדימה באמצעות $gt, ללא עלות $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה URL של העמוד (נוקה בצד השרת). |
| afterName | string | query | No | קרסור: העבר את nextAfterName מהתגובה הקודמת. (אופציונלי) |
| afterUserId | string | query | No | קישור קרסור למתי תקלות: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות זהים לא יפלו מהתוצאה. (אופציונלי) |

## Response

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'דוגמת getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Page URL identifier (cleaned server-side).
// מזהה URL של העמוד (נוקה בצד השרת).
let afterName = "afterName_example" // String | Cursor: pass nextAfterName from the previous response. (optional)
// קרסור: העבר את nextAfterName מהתגובה הקודמת. (אופציונלי)
let afterUserId = "afterUserId_example" // String | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. (optional)
// קישור קרסור למתי תקלות: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות זהים לא יפלו מהתוצאה. (אופציונלי)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]