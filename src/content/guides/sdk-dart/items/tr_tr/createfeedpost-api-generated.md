## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| broadcastId | string | query | Hayır |  |
| isLive | boolean | query | Hayır |  |
| doSpamCheck | boolean | query | Hayır |  |
| skipDupCheck | boolean | query | Hayır |  |

## Yanıt

Döndürür: `CreateFeedPostsResponse`

## Örnek

[inline-code-attrs-start title = 'createFeedPost Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdaki yorumu açarak API anahtarı için önek (örn. Bearer) ayarlayın, gerekirse
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