## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: `CreateModeratorResponse`

## Örnek

[inline-code-attrs-start title = 'createModerator Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// API anahtarı için önek (ör. Bearer) ayarlamak gerekiyorsa aşağıdakini yorumdan kaldırın
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createModeratorBody = CreateModeratorBody(); // CreateModeratorBody | 

try {
    final result = api_instance.createModerator(tenantId, createModeratorBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createModerator: $e\n');
}
[inline-code-end]