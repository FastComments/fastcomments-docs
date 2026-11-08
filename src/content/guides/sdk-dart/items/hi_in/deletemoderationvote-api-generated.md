## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| voteId | string | path | हाँ |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `VoteDeleteResponse`

## उदाहरण

[inline-code-attrs-start title = 'deleteModerationVote उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
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