## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| userId | string | query | Όχι |  |
| anonUserId | string | query | Όχι |  |

## Response

Επιστρέφει: `FlagCommentResponse`

## Example

[inline-code-attrs-start title = 'Παράδειγμα unFlagComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ξεσχολιάστε παρακάτω για να ορίσετε πρόθεμα (πχ. Bearer) για το κλειδί API, εφόσον απαιτείται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.unFlagComment(tenantId, id, UnFlagCommentOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->unFlagComment: $e\n');
}
[inline-code-end]

---