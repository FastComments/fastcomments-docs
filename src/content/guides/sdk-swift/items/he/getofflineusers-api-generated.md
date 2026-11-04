מגיבים בעבר בדף שאינם מחוברים כרגע. ממוינים לפי displayName.
השתמשו בזה לאחר אזילת /users/online כדי להציג מקטע "Members".
עימוד באמצעות סמן (cursor) על commenterName: השרת ממשיך לאורך האינדקס החלקי {tenantId, urlId, commenterName}
מ-afterName קדימה באמצעות $gt, ללא עלות $skip.

## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן | מזהה כתובת ה-URL של הדף (מנוקה בצד השרת). |
| afterName | string | query | לא | סמן: העבירו את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | לא | שובר שוויון לסמן: העבירו את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי ששמות זהים לא יגרמו להשמטת רשומות. |

## תגובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | מזהה כתובת ה-URL של הדף (מנוקה בצד השרת).
let afterName = "afterName_example" // String | סמן: העבירו את nextAfterName מהתגובה הקודמת. (optional)
let afterUserId = "afterUserId_example" // String | שובר שוויון לסמן: העבירו את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי ששמות זהים לא יגרמו להשמטת רשומות. (optional)

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