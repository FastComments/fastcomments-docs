## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Odgovor

Vraća: `CreateSubscriptionAPIResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer createSubscription'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPIUserSubscriptionData = CreateAPIUserSubscriptionData(); // CreateAPIUserSubscriptionData | 

try {
    final result = api_instance.createSubscription(tenantId, createAPIUserSubscriptionData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createSubscription: $e\n');
}
[inline-code-end]

---