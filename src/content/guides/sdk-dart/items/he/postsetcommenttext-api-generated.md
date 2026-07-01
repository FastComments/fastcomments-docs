## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | path | כן |  |
| broadcastId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: `SetCommentTextResponse`

## דוגמה

[inline-code-attrs-start title = 'postSetCommentText דוגמה'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final setCommentTextParams = SetCommentTextParams(); // SetCommentTextParams | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentText(tenantId, commentId, setCommentTextParams, PostSetCommentTextOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentText: $e\n');
}
[inline-code-end]