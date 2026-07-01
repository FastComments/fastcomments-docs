## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| commentId | string | path | Oui |  |
| dir | integer | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne : `GetCommentVoteUserNamesSuccessResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentVoteUserNames'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // Chaîne |
final commentId = commentId_example; // Chaîne |
final dir = 56; // entier |
final sso = sso_example; // Chaîne |

try {
    final result = api_instance.getCommentVoteUserNames(tenantId, commentId, dir, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentVoteUserNames: $e\n');
}
[inline-code-end]