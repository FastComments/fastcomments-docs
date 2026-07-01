req
tenantId
afterId

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|------------|----------|--------------|
| tenantId | string | query | Ja |  |
| afterId | string | query | Nej |  |
| limit | integer | query | Nej |  |
| tags | array | query | Nej |  |

## Svar

Returnerer: `GetFeedPostsResponse`

## Eksempel

[inline-code-attrs-start title = 'getFeedPosts Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøgle autorisation: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final afterId = afterId_example; // String | 
final limit = 56; // int | 
final tags = []; // List<String> | 

try {
    final result = api_instance.getFeedPosts(tenantId, GetFeedPostsOptions(afterId: afterId, limit: limit, tags: tags));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getFeedPosts: $e\n');
}
[inline-code-end]