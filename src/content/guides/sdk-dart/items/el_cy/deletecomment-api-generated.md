## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## Απάντηση

Returns: `DeleteCommentResult`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε την εξουσιοδότηση του κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ξεσχολιάστε παρακάτω για να ορίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final contextUserId = contextUserId_example; // String | 
final isLive = true; // bool | 

try {
    final result = api_instance.deleteComment(tenantId, id, DeleteCommentOptions(contextUserId: contextUserId, isLive: isLive));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteComment: $e\n');
}
[inline-code-end]