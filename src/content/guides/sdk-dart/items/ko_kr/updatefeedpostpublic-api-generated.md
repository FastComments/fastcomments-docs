## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | 문자열 | 경로 | 예 |  |
| postId | 문자열 | 경로 | 예 |  |
| broadcastId | 문자열 | 쿼리 | 아니오 |  |
| sso | 문자열 | 쿼리 | 아니오 |  |

## 응답

반환: `CreateFeedPostResponse`

## 예시

[inline-code-attrs-start title = 'updateFeedPostPublic 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postId = postId_example; // String | 
final updateFeedPostParams = UpdateFeedPostParams(); // UpdateFeedPostParams | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.updateFeedPostPublic(tenantId, postId, updateFeedPostParams, UpdateFeedPostPublicOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->updateFeedPostPublic: $e\n');
}
[inline-code-end]