## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|--------------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Retourne : `APIGetCommentResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommenter ci-dessous pour configurer le préfixe (p. ex. Bearer) de la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getComment(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getComment: $e\n');
}
[inline-code-end]

---