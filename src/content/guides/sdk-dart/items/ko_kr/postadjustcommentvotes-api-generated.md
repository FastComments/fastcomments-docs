## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|------|------|------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 응답

Returns: `AdjustVotesResponse`

## Example

[inline-code-attrs-start title = 'postAdjustCommentVotes 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
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