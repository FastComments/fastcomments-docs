## Paramètres

| Nom | Type | Location | Obligatoire | Description |
|------|------|----------|-------------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| errorId | string | path | Oui |  |

## Réponse

Renvoie : `APIEmptyResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteEmailTemplateRenderError'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final errorId = errorId_example; // String | 

try {
    final result = api_instance.deleteEmailTemplateRenderError(tenantId, id, errorId);
    print(result);
} catch (e) {
    print('Exception lors de l\'appel de DefaultApi->deleteEmailTemplateRenderError: $e\n');
}
[inline-code-end]