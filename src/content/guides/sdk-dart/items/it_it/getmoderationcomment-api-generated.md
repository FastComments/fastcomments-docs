## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| includeEmail | boolean | query | No |  |
| includeIP | boolean | query | No |  |
| sso | string | query | No |  |

## Response

Restituisce: `ModerationAPICommentResponse`

## Example

[inline-code-attrs-start title = 'Esempio getModerationComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final includeEmail = true; // bool | 
final includeIP = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getModerationComment(tenantId, commentId, GetModerationCommentOptions(includeEmail: includeEmail, includeIP: includeIP, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getModerationComment: $e\n');
}
[inline-code-end]