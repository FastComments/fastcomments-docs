## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: `AddSSOUserAPIResponse`

## Example

[inline-code-attrs-start title = 'addSSOUser Example'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
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
