## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Yanıt

Döndürür: `CreateQuestionResultResponse`

## Örnek

[inline-code-attrs-start title = 'createQuestionResult Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdakini yorumdan kaldırın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionResultBody = CreateQuestionResultBody(); // CreateQuestionResultBody | 

try {
    final result = api_instance.createQuestionResult(tenantId, createQuestionResultBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createQuestionResult: $e\n');
}
[inline-code-end]

---