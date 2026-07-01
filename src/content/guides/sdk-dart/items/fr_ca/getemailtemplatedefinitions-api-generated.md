## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Réponse

Renvoie : `GetEmailTemplateDefinitionsResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getEmailTemplateDefinitions'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l’autorisation de clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 

try {
    final result = api_instance.getEmailTemplateDefinitions(tenantId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getEmailTemplateDefinitions: $e\n');
}
[inline-code-end]

---