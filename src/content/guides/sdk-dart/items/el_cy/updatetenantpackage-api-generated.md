## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: `APIEmptyResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateTenantPackage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε την εξουσιοδότηση κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// αφαιρέστε το σχόλιο παρακάτω για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, εφόσον χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantPackageBody = UpdateTenantPackageBody(); // UpdateTenantPackageBody | 

try {
    final result = api_instance.updateTenantPackage(tenantId, id, updateTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenantPackage: $e\n');
}
[inline-code-end]