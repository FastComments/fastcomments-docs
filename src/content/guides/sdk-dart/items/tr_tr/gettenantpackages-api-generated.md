## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| skip | number | query | Hayır |  |

## Response

Döndürür: `GetTenantPackagesResponse`

## Örnek

[inline-code-attrs-start title = 'getTenantPackages Örnek'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdakini yorumdan çıkararak API anahtarı için önek (ör. Bearer) ayarlayın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantPackages(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantPackages: $e\n');
}
[inline-code-end]