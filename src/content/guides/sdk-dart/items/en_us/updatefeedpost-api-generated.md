## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Returns: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'updateFeedPost Example'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final feedPost = FeedPost(); // FeedPost | 

try {
    final result = api_instance.updateFeedPost(tenantId, id, feedPost);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateFeedPost: $e\n');
}
[inline-code-end]
