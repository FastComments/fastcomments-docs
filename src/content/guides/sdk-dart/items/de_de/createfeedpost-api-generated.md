## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | query | Ja |  |
| broadcastId | string | query | Nein |  |
| isLive | boolean | query | Nein |  |
| doSpamCheck | boolean | query | Nein |  |
| skipDupCheck | boolean | query | Nein |  |

## Antwort

Rückgabe: `CreateFeedPostsResponse`

## Beispiel

[inline-code-attrs-start title = 'createFeedPost Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Uncomment unten, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createFeedPostParams = CreateFeedPostParams(); // CreateFeedPostParams | 
final broadcastId = broadcastId_example; // String | 
final isLive = true; // bool | 
final doSpamCheck = true; // bool | 
final skipDupCheck = true; // bool | 

try {
    final result = api_instance.createFeedPost(tenantId, createFeedPostParams, CreateFeedPostOptions(broadcastId: broadcastId, isLive: isLive, doSpamCheck: doSpamCheck, skipDupCheck: skipDupCheck));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createFeedPost: $e\n');
}
[inline-code-end]