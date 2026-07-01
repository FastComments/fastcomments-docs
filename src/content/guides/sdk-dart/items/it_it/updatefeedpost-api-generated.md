## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: `APIEmptyResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio updateFeedPost'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurare l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// decommentare qui sotto per impostare il prefisso (es. Bearer) per la chiave API, se necessario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final feedPost = FeedPost(); // FeedPost | 

try {
    final result = api_instance.updateFeedPost(tenantId, id, feedPost);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateFeedPost: $e\n');
}
[inline-code-end]