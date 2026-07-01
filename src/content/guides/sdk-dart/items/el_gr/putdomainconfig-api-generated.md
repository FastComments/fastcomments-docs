## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Απόκριση

Επιστρέφει: `PutDomainConfigResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα putDomainConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε εξουσιοδότηση κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domainToUpdate = domainToUpdate_example; // String | 
final updateDomainConfigParams = UpdateDomainConfigParams(); // UpdateDomainConfigParams | 

try {
    final result = api_instance.putDomainConfig(tenantId, domainToUpdate, updateDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->putDomainConfig: $e\n');
}
[inline-code-end]

---