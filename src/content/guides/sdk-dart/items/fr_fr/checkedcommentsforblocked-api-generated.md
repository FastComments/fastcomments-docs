## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentIds | string | query | Yes | Une liste d'identifiants de commentaires séparés par des virgules. |
| sso | string | query | No |  |

## Réponse

Renvoie : `CheckBlockedCommentsResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de checkedCommentsForBlocked'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Une liste d'identifiants de commentaires séparés par des virgules.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]

---