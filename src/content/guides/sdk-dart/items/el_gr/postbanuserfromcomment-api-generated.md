## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | query | Ναι |  |
| commentId | string | path | Ναι |  |
| banEmail | boolean | query | Όχι |  |
| banEmailDomain | boolean | query | Όχι |  |
| banIP | boolean | query | Όχι |  |
| deleteAllUsersComments | boolean | query | Όχι |  |
| bannedUntil | string | query | Όχι |  |
| isShadowBan | boolean | query | Όχι |  |
| updateId | string | query | Όχι |  |
| banReason | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: `BanUserFromCommentResult`

## Παράδειγμα

[inline-code-attrs-start title = 'postBanUserFromComment Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final banEmail = true; // bool | 
final banEmailDomain = true; // bool | 
final banIP = true; // bool | 
final deleteAllUsersComments = true; // bool | 
final bannedUntil = bannedUntil_example; // String | 
final isShadowBan = true; // bool | 
final updateId = updateId_example; // String | 
final banReason = banReason_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postBanUserFromComment(tenantId, commentId, PostBanUserFromCommentOptions(banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postBanUserFromComment: $e\n');
}
[inline-code-end]

---