## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | query | Ναι |  |
| badgesUserId | string | query | Όχι |  |
| commentId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: `GetUserManualBadgesResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getManualBadgesForUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final badgesUserId = badgesUserId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getManualBadgesForUser(tenantId, GetManualBadgesForUserOptions(badgesUserId: badgesUserId, commentId: commentId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getManualBadgesForUser: $e\n');
}
[inline-code-end]