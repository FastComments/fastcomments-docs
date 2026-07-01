## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Odgovor

Vrne: `UpdateSubscriptionAPIResponse`

## Primer

[inline-code-attrs-start title = 'updateSubscription Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte avtentikacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPIUserSubscriptionData = UpdateAPIUserSubscriptionData(); // UpdateAPIUserSubscriptionData | 
final userId = userId_example; // String | 

try {
    final result = api_instance.updateSubscription(tenantId, id, updateAPIUserSubscriptionData, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateSubscription: $e\n');
}
[inline-code-end]

---