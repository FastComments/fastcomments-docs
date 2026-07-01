## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | requête | Oui |  |
| commentId | string | requête | Non |  |
| sso | string | requête | Non |  |

## Réponse

Retourne : `GetUserInternalProfileResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getUserInternalProfile'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserInternalProfile(tenantId, GetUserInternalProfileOptions(commentId: commentId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getUserInternalProfile: $e\n');
}
[inline-code-end]

---