## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| doSpamCheck | boolean | query | No |  |
| isLive | boolean | query | No |  |

## Odgovor

Returns: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'updateComment Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updatableCommentParams = UpdatableCommentParams(); // UpdatableCommentParams | 
final contextUserId = contextUserId_example; // String | 
final doSpamCheck = true; // bool | 
final isLive = true; // bool | 

try {
    final result = api_instance.updateComment(tenantId, id, updatableCommentParams, UpdateCommentOptions(contextUserId: contextUserId, doSpamCheck: doSpamCheck, isLive: isLive));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateComment: $e\n');
}
[inline-code-end]