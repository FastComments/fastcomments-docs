## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Response

Returns: `BlockSuccess`

## Example

[inline-code-attrs-start title = 'blockUserFromComment Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebny
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final blockFromCommentParams = BlockFromCommentParams(); // BlockFromCommentParams | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.blockUserFromComment(tenantId, id, blockFromCommentParams, BlockUserFromCommentOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->blockUserFromComment: $e\n');
}
[inline-code-end]

---