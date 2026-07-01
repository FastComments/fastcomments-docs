## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Yanıt

Döndürür: `GetModeratorResponse`

## Örnek

[inline-code-attrs-start title = 'getModerator Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// gerektiğinde API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdakini yorum kaldırın
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getModerator(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getModerator: $e\n');
}
[inline-code-end]