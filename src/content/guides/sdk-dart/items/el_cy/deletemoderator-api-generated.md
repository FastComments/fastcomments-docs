## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|-----------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| sendEmail | string | query | No |  |

## Απάντηση

Επιστρέφει: `APIEmptyResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'deleteModerator Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε την εξουσιοδότηση κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Αποσχολιάστε παρακάτω για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final sendEmail = sendEmail_example; // String | 

try {
    final result = api_instance.deleteModerator(tenantId, id, sendEmail);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteModerator: $e\n');
}
[inline-code-end]