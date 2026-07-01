## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| voteId | string | path | כן |  |
| urlId | string | query | כן |  |
| broadcastId | string | query | כן |  |
| editKey | string | query | לא |  |
| sso | string | query | לא |  |

## תשובה

מחזיר: `VoteDeleteResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteCommentVote'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final voteId = voteId_example; // String | 
final urlId = urlId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, DeleteCommentVoteOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteCommentVote: $e\n');
}
[inline-code-end]