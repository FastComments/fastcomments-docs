## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: `CreateHashTagResponse`

## Example

[inline-code-attrs-start title = 'addHashTag 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 필요에 따라 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하세요
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createHashTagBody = CreateHashTagBody(); // CreateHashTagBody | 

try {
    final result = api_instance.addHashTag(tenantId, createHashTagBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addHashTag: $e\n');
}
[inline-code-end]