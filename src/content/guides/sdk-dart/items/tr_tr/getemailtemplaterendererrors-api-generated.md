## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| skip | number | query | Hayır |  |

## Response

Döndürür: `GetEmailTemplateRenderErrorsResponse`

## Örnek

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdakini yorum dışı bırakın API anahtarı için önek (örn. Bearer) ayarlamak için, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getEmailTemplateRenderErrors(tenantId, id, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getEmailTemplateRenderErrors: $e\n');
}
[inline-code-end]