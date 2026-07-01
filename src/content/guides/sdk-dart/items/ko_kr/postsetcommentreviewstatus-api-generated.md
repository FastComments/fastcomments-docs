## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| reviewed | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 응답

반환: `APIEmptyResponse`

## 예시

[inline-code-attrs-start title = 'postSetCommentReviewStatus 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final reviewed = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentReviewStatus(tenantId, commentId, PostSetCommentReviewStatusOptions(reviewed: reviewed, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentReviewStatus: $e\n');
}
[inline-code-end]