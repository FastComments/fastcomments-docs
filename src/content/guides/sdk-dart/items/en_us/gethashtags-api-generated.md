## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | number | query | No |  |

## Response

Returns: `GetHashTagsResponse`

## Example

[inline-code-attrs-start title = 'getHashTags Example'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final page = 1.2; // double | 

try {
    final result = api_instance.getHashTags(tenantId, page);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getHashTags: $e\n');
}
[inline-code-end]
