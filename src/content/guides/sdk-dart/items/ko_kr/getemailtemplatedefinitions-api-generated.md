## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: `GetEmailTemplateDefinitionsResponse`

## 예제

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래 주석을 해제하여 API 키 접두사(예: Bearer)를 설정하세요, 필요 시
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 

try {
    final result = api_instance.getEmailTemplateDefinitions(tenantId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getEmailTemplateDefinitions: $e\n');
}
[inline-code-end]