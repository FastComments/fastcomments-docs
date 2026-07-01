## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|--------------|-------------|
| tenantId | string | query | Oui |  |
| tag | string | path | Oui |  |

## Réponse

Retourne : `UpdateHashTagResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple patchHashTag'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommentez ci‑dessous pour configurer le préfixe (p. ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final updateHashTagBody = UpdateHashTagBody(); // UpdateHashTagBody | 

try {
    final result = api_instance.patchHashTag(tenantId, tag, updateHashTagBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchHashTag: $e\n');
}
[inline-code-end]

---