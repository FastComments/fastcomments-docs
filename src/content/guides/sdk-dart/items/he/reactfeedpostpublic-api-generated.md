## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| isUndo | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## תגובה

מחזיר: `ReactFeedPostResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת reactFeedPostPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // מחרוזת | 
final postId = postId_example; // מחרוזת | 
final reactBodyParams = ReactBodyParams(); // ReactBodyParams | 
final isUndo = true; // בוליאני | 
final broadcastId = broadcastId_example; // מחרוזת | 
final sso = sso_example; // מחרוזת | 

try {
    final result = api_instance.reactFeedPostPublic(tenantId, postId, reactBodyParams, ReactFeedPostPublicOptions(isUndo: isUndo, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->reactFeedPostPublic: $e\n');
}
[inline-code-end]