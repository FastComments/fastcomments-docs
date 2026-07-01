## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | number | query | No |  |

## Respons

Returns: `GetHashTagsResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getHashTags Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer autorisatie voor API-sleutel: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ontcommentaar hieronder om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final page = 1.2; // double | 

try {
    final result = api_instance.getHashTags(tenantId, page);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getHashTags: $e\n');
}
[inline-code-end]