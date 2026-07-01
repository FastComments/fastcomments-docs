## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | path | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: `GetCommentBanStatusResponse`

## 예시

[inline-code-attrs-start title = 'getCommentBanStatus 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCommentBanStatus(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getCommentBanStatus: $e\n');
}
[inline-code-end]