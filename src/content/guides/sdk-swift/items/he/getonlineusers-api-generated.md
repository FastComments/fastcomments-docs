צופים מחוברים כרגע בעמוד: אנשים שסשן ה-websocket שלהם מנוי לעמוד ברגע זה.
מחזיר anonCount + totalCount (מנויים בחדר כולו, כולל צופים אנונימיים שאותם איננו מפרטים).

## Parameters

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | נתיב | כן |  |
| urlId | string | שאילתה | כן | מזהה URL של העמוד (מנוקה בצד השרת). |
| afterName | string | שאילתה | לא | מצביע: העבר את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | שאילתה | לא | מפסק שוויון של המצביע: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות זהים לא יגרמו להשמטת פריטים. |

## Response

מחזיר: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'דוגמה ל-getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, נא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | מזהה URL של העמוד (מנוקה בצד השרת).
let afterName = "afterName_example" // String | מצביע: העבר את nextAfterName מהתגובה הקודמת. (אופציונלי)
let afterUserId = "afterUserId_example" // String | מפסק-שוויון של המצביע: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות זהים לא יגרמו להשמטת פריטים. (אופציונלי)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]