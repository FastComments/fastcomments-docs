## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| locale | string | query | Non |  |

## Réponse

Renvoie : `RenderEmailTemplateResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple renderEmailTemplate'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé d'API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommentez ci‑dessous pour configurer le préfixe (p. ex. Bearer) pour la clé d'API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final renderEmailTemplateBody = RenderEmailTemplateBody(); // RenderEmailTemplateBody | 
final locale = locale_example; // String | 

try {
    final result = api_instance.renderEmailTemplate(tenantId, renderEmailTemplateBody, locale);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->renderEmailTemplate: $e\n');
}
[inline-code-end]