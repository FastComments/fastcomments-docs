## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|--------------|-------------|
| tenantId | string | query | Yes |  |
| tag | string | path | Yes |  |

## Réponse

Renvoie : `APIEmptyResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteHashTag'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommenter ci-dessous pour configurer le préfixe (ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final deleteHashTagRequestBody = DeleteHashTagRequestBody(); // DeleteHashTagRequestBody | 

try {
    final result = api_instance.deleteHashTag(tenantId, tag, deleteHashTagRequestBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteHashTag: $e\n');
}
[inline-code-end]