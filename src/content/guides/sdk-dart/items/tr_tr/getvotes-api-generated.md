## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |

## Yanıt

Returns: `GetVotesResponse`

## Örnek

[inline-code-attrs-start title = 'getVotes Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdakini yorumdan çıkararak API anahtarı için önek (örn. Bearer) ayarlayın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.getVotes(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getVotes: $e\n');
}
[inline-code-end]