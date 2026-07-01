## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-------------|
| tenantId | string | query | Ναι |  |
| tag | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: `UpdateHashTagResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patchHashTag'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε την εξουσιοδότηση κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Αποσχολιάστε παρακάτω για να καθορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final updateHashTagBody = UpdateHashTagBody(); // UpdateHashTagBody | 

try {
    final result = api_instance.patchHashTag(tenantId, tag, updateHashTagBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchHashTag: $e\n');
}
[inline-code-end]

---