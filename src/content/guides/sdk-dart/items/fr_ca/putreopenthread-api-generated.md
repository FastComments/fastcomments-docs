## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne : `APIEmptyResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple putReopenThread'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putReopenThread(tenantId, urlId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putReopenThread: $e\n');
}
[inline-code-end]