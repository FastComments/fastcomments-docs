req
tenantId
urlId
userIdWS

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| userIdWS | string | query | כן |  |
| startTime | integer | query | כן |  |
| endTime | integer | query | לא |  |

## תגובה

מחזיר: `GetEventLogResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת getGlobalEventLog'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final userIdWS = userIdWS_example; // String | 
final startTime = 789; // int | 
final endTime = 789; // int | 

try {
    final result = api_instance.getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getGlobalEventLog: $e\n');
}
[inline-code-end]

---