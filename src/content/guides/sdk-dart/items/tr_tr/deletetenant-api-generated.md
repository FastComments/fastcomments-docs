## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| sure | string | query | Hayır |  |

## Response

Döndürür: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'deleteTenant Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// aşağıdakini yorum dışı bırakın API anahtarının önekini (örn. Bearer) ayarlamak için, gerekirse
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final sure = sure_example; // String | 

try {
    final result = api_instance.deleteTenant(tenantId, id, sure);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTenant: $e\n');
}
[inline-code-end]