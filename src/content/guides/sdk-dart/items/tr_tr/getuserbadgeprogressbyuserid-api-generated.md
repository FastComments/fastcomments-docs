## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | path | Evet |  |

## Yanıt

Döndürür: `APIGetUserBadgeProgressResponse`

## Örnek

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Örnek'; type = ''; isFunctional = false; inline-code-attrs-end]
```dart
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Aşağıdakini yorumdan kaldırarak API anahtarı için ön ek (ör. Bearer) ayarlayın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressByUserId(tenantId, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressByUserId: $e\n');
}
```