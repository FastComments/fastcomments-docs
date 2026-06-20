עבר commenters בעמוד שאינם כרגע מקוונים. ממוין לפי displayName.
השתמש בזה לאחר שמיצית את /users/online כדי להציג מדור "חברים".
דפי-תוצאה מבוססי סמן על commenterName: השרת סורק את האינדקס החלקי {tenantId, urlId, commenterName} מ-afterName קדימה באמצעות $gt, ללא עלות של $skip.

## Parameters

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן | מזהה URL של הדף (מעובד בצד השרת). |
| afterName | string | query | לא | סמן: העבר את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | לא | מפסק תיקו של הסמן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שלא יאבדו פריטים כשיש שמות זהים. |

## תגובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדוגמאות הבאות עדיין בבטא. לכל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | מזהה URL של הדף (מעובד בצד השרת).
let afterName = "afterName_example" // String | סמן: העבר את nextAfterName מהתגובה הקודמת. (אופציונלי)
let afterUserId = "afterUserId_example" // String | מפסק תיקו של הסמן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שלא יאבדו פריטים כשיש שמות זהים. (אופציונלי)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]