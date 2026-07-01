## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |

## Respons

Returns: `GetVotesResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getVotes Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutautorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ontcomment de onderstaande regel om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.getVotes(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getVotes: $e\n');
}
[inline-code-end]