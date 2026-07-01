Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה URL של הדף (מנוקה בצד השרת). |
| afterName | string | query | No | סמן: העבר nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | מחלקת קישוריות של סמן: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שהקשרים על שם לא יפספסו ערכים. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'דוגמת getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדוגמאות של הקוד שלהלן עדיין בגרסת בטא. עבור כל בעיה, אנא דווח בכתובת http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | מזהה URL של הדף (מנוקה בצד השרת).
let afterName = "afterName_example" // String | סמן: העבר nextAfterName מהתגובה הקודמת. (optional)
let afterUserId = "afterUserId_example" // String | מחלקת קישוריות של סמן: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שהקשרים על שם לא יפספסו ערכים. (optional)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]