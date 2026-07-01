## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: `GetBannedUsersCountResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת getCounts'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCounts(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getCounts: $e\n');
}
[inline-code-end]