מגיבים קודמים בעמוד שאינם מחוברים כרגע. ממוינים לפי displayName.
השתמשו בזה לאחר אזילת /users/online כדי להציג מקטע "חברים".
עימוד סמן על commenterName: השרת מתקדם באינדקס החלקי {tenantId, urlId, commenterName}
מ-afterName קדימה באמצעות $gt, ללא עלות $skip.

## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן | מזהה כתובת ה-URL של העמוד (מנוקה בצד השרת). |
| afterName | string | query | לא | סמן: העבירו את nextAfterName מהתשובה הקודמת. |
| afterUserId | string | query | לא | שובר שוויון של הסמן: העבירו את nextAfterUserId מהתשובה הקודמת. נדרש כאשר afterName מוגדר כדי ששמות זהים לא יגרמו להשמטת רשומות. |

## תשובה

מחזיר: `PageUsersOfflineResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | מזהה כתובת ה-URL של העמוד (מנוקה בצד השרת).
final afterName = afterName_example; // String | סמן: העבירו את nextAfterName מהתשובה הקודמת.
final afterUserId = afterUserId_example; // String | שובר שוויון של הסמן: העבירו את nextAfterUserId מהתשובה הקודמת. נדרש כאשר afterName מוגדר כדי ששמות זהים לא יגרמו להשמטת רשומות.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]