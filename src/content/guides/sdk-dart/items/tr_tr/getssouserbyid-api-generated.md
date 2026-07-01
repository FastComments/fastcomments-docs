## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: `GetSSOUserByIdAPIResponse`

## Örnek

[inline-code-attrs-start title = 'getSSOUserById Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı kimlik doğrulamasını yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Aşağıdakini API anahtarına önek (ör. Bearer) eklemek için yorum satırından çıkar, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getSSOUserById(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUserById: $e\n');
}
[inline-code-end]

---