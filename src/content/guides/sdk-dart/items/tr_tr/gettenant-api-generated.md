## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: `GetTenantResponse`

## Örnek

[inline-code-attrs-start title = 'getTenant Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Aşağıdaki satırın yorumunu kaldırarak API anahtarı için önek ayarlayın (örn. Bearer), gerekirse

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getTenant(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenant: $e\n');
}
[inline-code-end]