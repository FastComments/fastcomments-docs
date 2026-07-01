## Parametreler

| İsim | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: `CreateEmailTemplateResponse`

## Örnek

[inline-code-attrs-start title = 'createEmailTemplate Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdakini yorumdan çıkararak API anahtarı için önek (örn. Bearer) ayarlayın, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createEmailTemplateBody = CreateEmailTemplateBody(); // CreateEmailTemplateBody | 

try {
    final result = api_instance.createEmailTemplate(tenantId, createEmailTemplateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createEmailTemplate: $e\n');
}
[inline-code-end]