## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | path | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Returns: `GetBannedUsersFromCommentResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getBanUsersFromComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getBanUsersFromComment(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getBanUsersFromComment: $e\n');
}
[inline-code-end]