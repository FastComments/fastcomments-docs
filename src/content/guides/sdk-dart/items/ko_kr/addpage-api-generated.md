## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

반환: `AddPageAPIResponse`

## Example

[inline-code-attrs-start title = 'addPage 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPIPageData = CreateAPIPageData(); // CreateAPIPageData | 

try {
    final result = api_instance.addPage(tenantId, createAPIPageData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addPage: $e\n');
}
[inline-code-end]