## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Yanıt

Döndürür: `APIEmptySuccessResponse`

## Örnek

[inline-code-attrs-start title = 'updateUserBadge Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// API anahtarı için ön ek (ör. Bearer) ayarlamak için aşağıdakini yorumdan çıkartın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateUserBadgeParams = UpdateUserBadgeParams(); // UpdateUserBadgeParams | 

try {
    final result = api_instance.updateUserBadge(tenantId, id, updateUserBadgeParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateUserBadge: $e\n');
}
[inline-code-end]