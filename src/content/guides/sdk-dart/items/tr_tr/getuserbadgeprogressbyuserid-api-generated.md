## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | path | Evet |  |

## Yanıt

Döndürür: `APIGetUserBadgeProgressResponse`

## Örnek

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtar yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// gerekirse API anahtarı için önek (örn. Bearer) ayarlamak üzere aşağıdaki satırın yorumunu kaldırın
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressByUserId(tenantId, userId);
    print(result);
} catch (e) {
    print('DefaultApi->getUserBadgeProgressByUserId çağrılırken istisna oluştu: $e\n');
}
[inline-code-end]