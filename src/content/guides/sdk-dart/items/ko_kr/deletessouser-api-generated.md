## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## 응답

Returns: `DeleteSSOUserAPIResponse`

## 예시

[inline-code-attrs-start title = 'deleteSSOUser 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래를 주석 해제하면 필요 시 API 키에 대한 접두사(예: Bearer)를 설정합니다
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final deleteComments = true; // bool | 
final commentDeleteMode = commentDeleteMode_example; // String | 

try {
    final result = api_instance.deleteSSOUser(tenantId, id, DeleteSSOUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteSSOUser: $e\n');
}
[inline-code-end]