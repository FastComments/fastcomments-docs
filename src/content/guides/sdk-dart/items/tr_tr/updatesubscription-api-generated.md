## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| userId | string | query | Hayır |  |

## Response

Döndürür: `UpdateSubscriptionAPIResponse`

## Örnek

[inline-code-attrs-start title = 'updateSubscription Örnek'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdaki satırı yorum dışı bırakın ve API anahtarı için önek (örn. Bearer) ayarlayın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPIUserSubscriptionData = UpdateAPIUserSubscriptionData(); // UpdateAPIUserSubscriptionData | 
final userId = userId_example; // String | 

try {
    final result = api_instance.updateSubscription(tenantId, id, updateAPIUserSubscriptionData, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateSubscription: $e\n');
}
[inline-code-end]