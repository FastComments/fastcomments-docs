## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | integer | query | No |  |

## Odgovor

Vraća: `GetSSOUsersResponse`

## Primer

[inline-code-attrs-start title = 'Primer getSSOUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Подесите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentarišite ispod da postavite префикс (нпр. Bearer) за API кључ, ако је потребно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 56; // int | 

try {
    final result = api_instance.getSSOUsers(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUsers: $e\n');
}
[inline-code-end]