## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentIds | string | query | כן | רשימה מופרדת בפסיקים של מזהי תגובות. |
| sso | string | query | לא |  |

## תגובה

מחזיר: `CheckBlockedCommentsResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת checkedCommentsForBlocked'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | רשימה מופרדת בפסיקים של מזהי תגובות.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]