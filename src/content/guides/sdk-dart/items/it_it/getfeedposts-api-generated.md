req
tenantId
afterId

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |

## Risposta

Restituisce: `GetFeedPostsResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio getFeedPosts'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configura l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// decommenta qui sotto per impostare il prefisso (ad es. Bearer) per la chiave API, se necessario
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