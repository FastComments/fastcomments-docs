## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| broadcastId | string | query | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

Returns: `APIEmptyResponse`

## 예시

[inline-code-attrs-start title = 'unLockComment 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.unLockComment(tenantId, commentId, broadcastId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->unLockComment: $e\n');
}
[inline-code-end]