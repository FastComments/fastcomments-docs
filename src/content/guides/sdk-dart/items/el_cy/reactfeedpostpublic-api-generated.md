## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| postId | string | path | Ναι |  |
| isUndo | boolean | query | Όχι |  |
| broadcastId | string | query | Όχι |  |
| sSO | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: `ReactFeedPostResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα reactFeedPostPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postId = postId_example; // String | 
final reactBodyParams = ReactBodyParams(); // ReactBodyParams | 
final isUndo = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.reactFeedPostPublic(tenantId, postId, reactBodyParams, ReactFeedPostPublicOptions(isUndo: isUndo, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->reactFeedPostPublic: $e\n');
}
[inline-code-end]

---