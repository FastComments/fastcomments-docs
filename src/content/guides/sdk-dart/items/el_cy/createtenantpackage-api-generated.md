## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Απάντηση

Επιστρέφει: `CreateTenantPackageResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createTenantPackage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε την εξουσιοδότηση κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// καταργήστε το σχόλιο παρακάτω για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantPackageBody = CreateTenantPackageBody(); // CreateTenantPackageBody | 

try {
    final result = api_instance.createTenantPackage(tenantId, createTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenantPackage: $e\n');
}
[inline-code-end]