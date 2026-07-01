## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| doSpamCheck | boolean | query | No |  |
| isLive | boolean | query | No |  |

## Response

Returns: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'updateComment Example'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updatableCommentParams = UpdatableCommentParams(); // UpdatableCommentParams | 
final contextUserId = contextUserId_example; // String | 
final doSpamCheck = true; // bool | 
final isLive = true; // bool | 

try {
    final result = api_instance.updateComment(tenantId, id, updatableCommentParams, UpdateCommentOptions(contextUserId: contextUserId, doSpamCheck: doSpamCheck, isLive: isLive));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateComment: $e\n');
}
[inline-code-end]
