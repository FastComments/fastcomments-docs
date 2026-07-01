## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Απόκριση

Returns: `DeletePageAPIResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deletePage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε εξουσιοδότηση κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Αναίρεση σχολιασμού παρακάτω για ρύθμιση προθέματος (π.χ. Bearer) για το κλειδί API, εφόσον χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deletePage(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deletePage: $e\n');
}
[inline-code-end]