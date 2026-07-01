## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: `APIGetUserBadgeProgressResponse`

## Örnek

[inline-code-attrs-start title = 'getUserBadgeProgressById Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdakini yorumdan çıkarın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressById(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressById: $e\n');
}
[inline-code-end]