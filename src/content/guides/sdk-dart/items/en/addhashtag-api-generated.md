## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: `CreateHashTagResponse`

## Example

[inline-code-attrs-start title = 'addHashTag Example'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
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
