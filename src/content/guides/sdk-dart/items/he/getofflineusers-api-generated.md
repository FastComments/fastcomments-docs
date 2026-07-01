Past commenters on the page who are NOT currently online. Sorted by displayName.  
מגיבים קודמים בדף שאין בהם כרגע במצב מקוון. ממוינים לפי displayName.  

Use this after exhausting /users/online to render a "Members" section.  
השתמש בזה לאחר מריחת /users/online כדי ליצור סעיף "Members".  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
דפדוף בעזרת מצביע (cursor) על commenterName: השרת עובר על החלק החלקי {tenantId, urlId, commenterName} מ‑afterName קדימה באמצעות $gt, ללא עלות $skip.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה כתובת URL של העמוד (נוקה בצד השרת). |
| afterName | string | query | No | מצביע: העבר nextAfterName מתשובה קודמת. |
| afterUserId | string | query | No | קיבוע מצביע במקרים של שוויון: העבר nextAfterUserId מתשובה קודמת. נדרש כאשר afterName מוגדר כך שמקרים של שוויון בשם לא יוותרו על רשומות. |

## תגובה

Returns: `PageUsersOfflineResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | מזהה כתובת URL של העמוד (נוקה בצד השרת).
final afterName = afterName_example; // String | מצביע: העבר nextAfterName מתשובה קודמת.
final afterUserId = afterUserId_example; // String | קיבוע מצביע במקרים של שוויון: העבר nextAfterUserId מתשובה קודמת. נדרש כאשר afterName מוגדר כך שמקרים של שוויון בשם לא יוותרו על רשומות.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]