## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: `GetQuestionConfigsResponse`

## Örnek

[inline-code-attrs-start title = 'getQuestionConfigs Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// API anahtarı için (ör. Bearer) önek ayarlamak için aşağıdakini yorum dışı bırakın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getQuestionConfigs(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionConfigs: $e\n');
}
[inline-code-end]

---