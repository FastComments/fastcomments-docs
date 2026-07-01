## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |

## 응답

Returns: `APIEmptyResponse`

## 예시

[inline-code-attrs-start title = 'deleteEmailTemplate 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래의 주석을 해제하여 필요 시 API 키에 대한 프리픽스(e.g. Bearer)를 설정하세요
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteEmailTemplate(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteEmailTemplate: $e\n');
}
[inline-code-end]