## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |

## Response

Retourneert: `GetDomainConfigsResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getDomainConfigs Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// verwijder de commentaartekens hieronder om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 

try {
    final result = api_instance.getDomainConfigs(tenantId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getDomainConfigs: $e\n');
}
[inline-code-end]