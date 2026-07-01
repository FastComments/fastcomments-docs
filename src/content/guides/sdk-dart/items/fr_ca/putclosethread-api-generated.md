## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Returns: `APIEmptyResponse`

## Exemple

[inline-code-attrs-start title = 'putCloseThread Exemple'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // Chaîne | 
final urlId = urlId_example; // Chaîne | 
final sso = sso_example; // Chaîne | 

try {
    final result = api_instance.putCloseThread(tenantId, urlId, sno);
    print(result);
} catch (e) {
    print('Exception lors de l\'appel de ModerationApi->putCloseThread: $e\n');
}
[inline-code-end]