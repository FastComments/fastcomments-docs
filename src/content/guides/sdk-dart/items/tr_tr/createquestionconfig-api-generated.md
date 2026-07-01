## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Yanıt

Döndürür: `CreateQuestionConfigResponse`

## Örnek

[inline-code-attrs-start title = 'createQuestionConfig Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdakini yorum dışı bırakarak API anahtarı için önek (örn. Bearer) ayarlayın, gerekiyorsa
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionConfigBody = CreateQuestionConfigBody(); // CreateQuestionConfigBody | 

try {
    final result = api_instance.createQuestionConfig(tenantId, createQuestionConfigBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createQuestionConfig: $e\n');
}
[inline-code-end]

---