## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | sorgu | Evet |  |

## Yanıt

Döndürür: `AddPageAPIResponse`

## Örnek

[inline-code-attrs-start title = 'addPage Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// gerektiğinde API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdakini yorumdan çıkar
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPIPageData = CreateAPIPageData(); // CreateAPIPageData | 

try {
    final result = api_instance.addPage(tenantId, createAPIPageData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addPage: $e\n');
}
[inline-code-end]