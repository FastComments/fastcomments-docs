## Parameters

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|-----------|------|
| tenantId | string | poizvedba | Da |  |
| commentId | string | pot | Da |  |
| banEmail | boolean | poizvedba | Ne |  |
| banEmailDomain | boolean | poizvedba | Ne |  |
| banIP | boolean | poizvedba | Ne |  |
| deleteAllUsersComments | boolean | poizvedba | Ne |  |
| bannedUntil | string | poizvedba | Ne |  |
| isShadowBan | boolean | poizvedba | Ne |  |
| updateId | string | poizvedba | Ne |  |
| banReason | string | poizvedba | Ne |  |
| sso | string | poizvedba | Ne |  |

## Response

Vrne: `BanUserFromCommentResult`

## Example

[inline-code-attrs-start title = 'postBanUserFromComment Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // Niz | 
final commentId = commentId_example; // Niz | 
final banEmail = true; // bool | 
final banEmailDomain = true; // bool | 
final banIP = true; // bool | 
final deleteAllUsersComments = true; // bool | 
final bannedUntil = bannedUntil_example; // Niz | 
final isShadowBan = true; // bool | 
final updateId = updateId_example; // Niz | 
final banReason = banReason_example; // Niz | 
final sso = sso_example; // Niz | 

try {
    final result = api_instance.postBanUserFromComment(tenantId, commentId, PostBanUserFromCommentOptions(banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postBanUserFromComment: $e\n');
}
[inline-code-end]