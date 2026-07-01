## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 응답

반환: `AddSSOUserAPIResponse`

## 예제

[inline-code-attrs-start title = 'addSSOUser 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증을 구성하십시오: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 필요에 따라 API 키에 대한 접두사(e.g. Bearer)를 설정하려면 아래 주석을 해제하십시오
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPISSOUserData = CreateAPISSOUserData(); // CreateAPISSOUserData | 

try {
    final result = api_instance.addSSOUser(tenantId, createAPISSOUserData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addSSOUser: $e\n');
}
[inline-code-end]