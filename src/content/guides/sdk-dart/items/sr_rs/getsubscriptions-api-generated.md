## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |

## Odgovor

Vraća: `GetSubscriptionsAPIResponse`

## Primer

[inline-code-attrs-start title = 'Primer getSubscriptions'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуришите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ukomentarišite donji red da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.getSubscriptions(tenantId, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSubscriptions: $e\n');
}
[inline-code-end]