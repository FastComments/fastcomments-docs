## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | query | Ja |  |
| email | string | path | Ja |  |

## Respons

Retourneert: `GetSSOUserByEmailAPIResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getSSOUserByEmail Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ontcommentarieer onderstaande om prefix (bijv. Bearer) voor API-sleutel in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final email = email_example; // String | 

try {
    final result = api_instance.getSSOUserByEmail(tenantId, email);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUserByEmail: $e\n');
}
[inline-code-end]