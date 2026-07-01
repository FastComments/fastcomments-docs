## Parameters

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |

## Response

Döndürür: `GetPageByURLIdAPIResponse`

## Example

[inline-code-attrs-start title = 'getPageByURLId Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// API anahtarı için ön ek (ör. Bearer) ayarlamak üzere aşağıdakini yorumdan çıkar, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.getPageByURLId(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPageByURLId: $e\n');
}
[inline-code-end]