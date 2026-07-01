## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|------------|------------|-------------|
| tenantId | string | query | Yes |  |

## Réponse

Renvoie : `AddDomainConfigResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple addDomainConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommenter ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final addDomainConfigParams = AddDomainConfigParams(); // AddDomainConfigParams | 

try {
    final result = api_instance.addDomainConfig(tenantId, addDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception lors de l\'appel de DefaultApi->addDomainConfig: $e\n');
}
[inline-code-end]