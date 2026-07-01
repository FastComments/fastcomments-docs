## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |

## Odgovor

Vraća: `GetSubscriptionsAPIResponse`

## Primer

[inline-code-attrs-start title = 'Primer getSubscriptions'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, po potrebi
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

---