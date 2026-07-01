## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| doSpamCheck | boolean | query | No |  |
| isLive | boolean | query | No |  |

## Risposta

Restituisce: `APIEmptyResponse`

## Esempio

[inline-code-attrs-start title = 'updateComment Esempio'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configura l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// rimuovi il commento qui sotto per impostare il prefisso (es. Bearer) per la chiave API, se necessario
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