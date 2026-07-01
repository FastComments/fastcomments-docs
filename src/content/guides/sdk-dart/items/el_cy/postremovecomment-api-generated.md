## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | path | Ναι |  |
| broadcastId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: `PostRemoveCommentApiResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'postRemoveComment Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postRemoveComment(tenantId, commentId, PostRemoveCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postRemoveComment: $e\n');
}
[inline-code-end]