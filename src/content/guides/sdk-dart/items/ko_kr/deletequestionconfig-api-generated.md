## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 쿼리 | 예 |  |
| id | string | 경로 | 예 |  |

## Response

반환: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'deleteQuestionConfig 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래 주석을 해제하여 API 키에 대한 접두사(예: Bearer)를 설정합니다, 필요하면
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // 문자열 | 
final id = id_example; // 문자열 | 

try {
    final result = api_instance.deleteQuestionConfig(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteQuestionConfig: $e\n');
}
[inline-code-end]