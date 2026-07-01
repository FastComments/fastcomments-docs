## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| isLive | boolean | query | Nee |  |
| doSpamCheck | boolean | query | Nee |  |
| sendEmails | boolean | query | Nee |  |
| populateNotifications | boolean | query | Nee |  |

## Respons

Retourneert: `APISaveCommentResponse`

## Voorbeeld

[inline-code-attrs-start title = 'saveComment Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ontcommentarieer onderstaande om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createCommentParams = CreateCommentParams(); // CreateCommentParams | 
final isLive = true; // bool | 
final doSpamCheck = true; // bool | 
final sendEmails = true; // bool | 
final populateNotifications = true; // bool | 

try {
    final result = api_instance.saveComment(tenantId, createCommentParams, SaveCommentOptions(isLive: isLive, doSpamCheck: doSpamCheck, sendEmails: sendEmails, populateNotifications: populateNotifications));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->saveComment: $e\n');
}
[inline-code-end]