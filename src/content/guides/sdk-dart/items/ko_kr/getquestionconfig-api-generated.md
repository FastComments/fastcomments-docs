## Parameters

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

반환: `GetQuestionConfigResponse`

## Example

[inline-code-attrs-start title = 'getQuestionConfig 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래의 주석을 해제하여 API 키에 대한 접두사 (예: Bearer)를 설정합니다, 필요한 경우
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getQuestionConfig(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionConfig: $e\n');
}
[inline-code-end]