## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 응답

Returns: `APIEmptyResponse`

## 예시

[inline-code-attrs-start title = 'postFlagComment 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // 문자열 |
final commentId = commentId_example; // 문자열 |
final broadcastId = broadcastId_example; // 문자열 |
final sso = sso_example; // 문자열 |

try {
    final result = api_instance.postFlagComment(tenantId, commentId, PostFlagCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postFlagComment: $e\n');
}
[inline-code-end]