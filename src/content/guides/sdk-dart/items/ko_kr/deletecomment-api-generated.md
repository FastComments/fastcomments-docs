## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| contextUserId | string | query | 아니오 |  |
| isLive | boolean | query | 아니오 |  |

## 응답

반환: `DeleteCommentResult`

## 예시

[inline-code-attrs-start title = 'deleteComment 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 필요할 경우 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하세요
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final contextUserId = contextUserId_example; // String | 
final isLive = true; // bool | 

try {
    final result = api_instance.deleteComment(tenantId, id, DeleteCommentOptions(contextUserId: contextUserId, isLive: isLive));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteComment: $e\n');
}
[inline-code-end]