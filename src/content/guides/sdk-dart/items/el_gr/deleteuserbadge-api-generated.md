## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Απάντηση

Επιστρέφει: `APIEmptySuccessResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteUserBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε την εξουσιοδότηση του κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Αφαίρεση σχολίου παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteUserBadge(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteUserBadge: $e\n');
}
[inline-code-end]

---