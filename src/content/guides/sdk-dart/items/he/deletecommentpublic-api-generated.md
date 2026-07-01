## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| broadcastId | string | query | כן |  |
| editKey | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: `PublicAPIDeleteCommentResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteCommentPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // מחרוזת | 
final commentId = commentId_example; // מחרוזת | 
final broadcastId = broadcastId_example; // מחרוזת | 
final editKey = editKey_example; // מחרוזת | 
final sso = sso_example; // מחרוזת | 

try {
    final result = api_instance.deleteCommentPublic(tenantId, commentId, broadcastId, DeleteCommentPublicOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteCommentPublic: $e\n');
}
[inline-code-end]