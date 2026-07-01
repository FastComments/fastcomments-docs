## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| contextUserId | string | query | Nee |  |
| doSpamCheck | boolean | query | Nee |  |
| isLive | boolean | query | Nee |  |

## Respons

Retourneert: `APIEmptyResponse`

## Voorbeeld

[inline-code-attrs-start title = 'updateComment Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutelautorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// haal onderstaande uit commentaar om prefix (bijv. Bearer) voor API-sleutel in te stellen, indien nodig
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