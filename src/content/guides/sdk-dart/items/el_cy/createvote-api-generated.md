## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| direction | string | query | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Απόκριση

Επιστρέφει: `VoteResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'createVote Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε εξουσιοδότηση κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Αποσχολιάστε παρακάτω για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final direction = direction_example; // String | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.createVote(tenantId, commentId, direction, CreateVoteOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createVote: $e\n');
}
[inline-code-end]