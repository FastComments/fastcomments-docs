## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Yanıt

Döndürür: `GetTicketResponse`

## Örnek

[inline-code-attrs-start title = 'getTicket Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Aşağıdaki satırı yorum dışı bırakarak API anahtarı için önek (örn. Bearer) ayarlayın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.getTicket(tenantId, id, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTicket: $e\n');
}
[inline-code-end]

---