## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Yanıt

Döndürür: `CreateHashTagResponse`

## Örnek

[inline-code-attrs-start title = 'addHashTag Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdakini yorum satırından çıkararak API anahtarı için önek (ör. Bearer) ayarlayın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createHashTagBody = CreateHashTagBody(); // CreateHashTagBody | 

try {
    final result = api_instance.addHashTag(tenantId, createHashTagBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addHashTag: $e\n');
}
[inline-code-end]