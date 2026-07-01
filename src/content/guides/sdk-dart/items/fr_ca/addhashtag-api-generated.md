## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |

## Réponse

Retourne : `CreateHashTagResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple addHashTag'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommentez ci-dessous pour configurer le préfixe (p. ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createHashTagBody = CreateHashTagBody(); // CreateHashTagBody | 

try {
    final result = api_instance.addHashTag(tenantId, createHashTagBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addHashTag: $e\n');
}
[inline-code-end]