## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: `CreateTenantUserResponse`

## 예시

[inline-code-attrs-start title = 'createTenantUser 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래 주석을 해제하여 API 키에 대한 접두사(e.g. Bearer)를 설정합니다, 필요 시
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantUserBody = CreateTenantUserBody(); // CreateTenantUserBody | 

try {
    final result = api_instance.createTenantUser(tenantId, createTenantUserBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenantUser: $e\n');
}
[inline-code-end]