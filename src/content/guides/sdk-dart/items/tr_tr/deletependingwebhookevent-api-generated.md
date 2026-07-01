## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Yanıt

Döndürür: `APIEmptyResponse`

## Örnek

[inline-code-attrs-start title = 'deletePendingWebhookEvent Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// API anahtarı için önek (ör. Bearer) ayarlamak istiyorsanız aşağıdakini yorumdan çıkarın
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deletePendingWebhookEvent(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deletePendingWebhookEvent: $e\n');
}
[inline-code-end]