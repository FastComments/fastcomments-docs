## Parametreler

| İsim | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| redirectURL | string | query | Hayır |  |

## Yanıt

Döndürür: `APIEmptyResponse`

## Örnek

[inline-code-attrs-start title = 'sendLoginLink Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandırın: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Gerekirse API anahtarı için ön ek (örn. Bearer) ayarlamak için aşağıdaki yorum satırını kaldırın
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final redirectURL = redirectURL_example; // String | 

try {
    final result = api_instance.sendLoginLink(tenantId, id, redirectURL);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->sendLoginLink: $e\n');
}
[inline-code-end]

---