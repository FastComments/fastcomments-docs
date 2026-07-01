## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: `GetBannedUsersFromCommentResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת getBanUsersFromComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getBanUsersFromComment(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getBanUsersFromComment: $e\n');
}
[inline-code-end]