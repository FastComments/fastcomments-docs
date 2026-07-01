## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-------------|
| tenantId | string | διαδρομή | Ναι |  |
| commentId | string | διαδρομή | Ναι |  |
| broadcastId | string | ερώτημα | Ναι |  |
| editKey | string | ερώτημα | Όχι |  |
| sso | string | ερώτημα | Όχι |  |

## Απάντηση

Επιστρέφει: `PublicAPIDeleteCommentResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'deleteCommentPublic Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteCommentPublic(tenantId, commentId, broadcastId, DeleteCommentPublicOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteCommentPublic: $e\n');
}
[inline-code-end]