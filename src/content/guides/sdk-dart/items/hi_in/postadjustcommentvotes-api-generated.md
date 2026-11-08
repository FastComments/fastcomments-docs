## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `AdjustVotesResponse`

## उदाहरण

[inline-code-attrs-start title = 'postAdjustCommentVotes उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final adjustCommentVotesParams = AdjustCommentVotesParams(); // AdjustCommentVotesParams | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postAdjustCommentVotes(tenantId, commentId, adjustCommentVotesParams, PostAdjustCommentVotesOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postAdjustCommentVotes: $e\n');
}
[inline-code-end]