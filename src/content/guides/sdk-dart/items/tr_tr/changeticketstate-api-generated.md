## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |
| id | string | path | Yes |  |

## Yanıt

Döndürür: `ChangeTicketStateResponse`

## Örnek

[inline-code-attrs-start title = 'changeTicketState Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdaki satırı açarak API anahtarı için önek (ör. Bearer) ayarlayın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final id = id_example; // String | 
final changeTicketStateBody = ChangeTicketStateBody(); // ChangeTicketStateBody | 

try {
    final result = api_instance.changeTicketState(tenantId, userId, id, changeTicketStateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->changeTicketState: $e\n');
}
[inline-code-end]