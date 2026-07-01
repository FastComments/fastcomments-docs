## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odgovor

Returns: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'Primer deleteNotificationCount'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Nastavi avtorizacijo ključa API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentiraj spodaj, da nastaviš predpono (npr. Bearer) za ključ API, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteNotificationCount(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteNotificationCount: $e\n');
}
[inline-code-end]