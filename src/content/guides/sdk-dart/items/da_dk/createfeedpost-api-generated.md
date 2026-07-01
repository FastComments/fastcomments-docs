## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| broadcastId | string | query | No |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| skipDupCheck | boolean | query | No |  |

## Respons

Returnerer: `CreateFeedPostResponse`

## Eksempel

[inline-code-attrs-start title = 'createFeedPost Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøgleautorisation: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøgle, hvis nødvendigt
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