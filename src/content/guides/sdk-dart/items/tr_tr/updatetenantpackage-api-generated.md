---
## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: `APIEmptyResponse`

## Örnek

[inline-code-attrs-start title = 'updateTenantPackage Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// API anahtarının ön ekini (ör. Bearer) ayarlamak için aşağıdakini yorum kaldırın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantPackageBody = UpdateTenantPackageBody(); // UpdateTenantPackageBody | 

try {
    final result = api_instance.updateTenantPackage(tenantId, id, updateTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenantPackage: $e\n');
}
[inline-code-end]

---