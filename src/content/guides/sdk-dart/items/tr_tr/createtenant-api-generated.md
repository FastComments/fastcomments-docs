## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Yanıt

Döndürür: `CreateTenantResponse`

## Örnek

[inline-code-attrs-start title = 'createTenant Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Aşağıdakinin yorumunu kaldırarak ön ek (ör. Bearer) ayarlayın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantBody = CreateTenantBody(); // CreateTenantBody | 

try {
    final result = api_instance.createTenant(tenantId, createTenantBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenant: $e\n');
}
[inline-code-end]