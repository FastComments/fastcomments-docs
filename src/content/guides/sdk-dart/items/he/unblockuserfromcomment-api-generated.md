## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |

## Response

תגובה: `UnblockSuccess`

## דוגמה

[inline-code-attrs-start title = 'דוגמת unBlockUserFromComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO הגדר הרשאת מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל את ההערה למטה כדי להגדיר קידומת (לדוגמה, Bearer) למפתח API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final unBlockFromCommentParams = UnBlockFromCommentParams(); // UnBlockFromCommentParams | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, UnBlockUserFromCommentOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->unBlockUserFromComment: $e\n');
}
[inline-code-end]