Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה כתובת URL של הדף (נוקה בצד השרת). |
| afterName | string | query | No | סמן: העבר nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | מפריד סמן: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקבצי שמות לא יורדו מהתוצאות. |

## תגובה

Returns: `PageUsersOnlineResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמה getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | מזהה כתובת URL של הדף (נוקה בצד השרת).
final afterName = afterName_example; // String | סמן: העבר nextAfterName מהתגובה הקודמת.
final afterUserId = afterUserId_example; // String | מפריד סמן: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקבצי שמות לא יורדו מהתוצאות.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]