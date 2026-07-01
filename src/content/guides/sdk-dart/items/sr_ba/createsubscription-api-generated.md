## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: `CreateSubscriptionAPIResponse`

## Primjer

[inline-code-attrs-start title = 'createSubscription Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
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