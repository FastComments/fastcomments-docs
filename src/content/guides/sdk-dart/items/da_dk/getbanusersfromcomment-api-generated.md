## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: `GetBannedUsersFromCommentResponse`

## Eksempel

[inline-code-attrs-start title = 'getBanUsersFromComment Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getBanUsersFromComment(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getBanUsersFromComment: $e\n');
}
[inline-code-end]