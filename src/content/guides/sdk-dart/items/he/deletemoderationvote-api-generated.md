## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | path | כן |  |
| voteId | string | path | כן |  |
| broadcastId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: `VoteDeleteResponse`

## דוגמה

[inline-code-attrs-start title = 'deleteModerationVote דוגמה'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final voteId = voteId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteModerationVote(tenantId, commentId, voteId, DeleteModerationVoteOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->deleteModerationVote: $e\n');
}
[inline-code-end]