## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| includeEmail | boolean | query | Nein |  |
| includeIP | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: `ModerationAPICommentResponse`

## Beispiel

[inline-code-attrs-start title = 'getModerationComment Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
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