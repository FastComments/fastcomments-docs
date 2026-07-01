## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| banEmail | boolean | query | Ne |  |
| banEmailDomain | boolean | query | Ne |  |
| banIP | boolean | query | Ne |  |
| deleteAllUsersComments | boolean | query | Ne |  |
| bannedUntil | string | query | Ne |  |
| isShadowBan | boolean | query | Ne |  |
| updateId | string | query | Ne |  |
| banReason | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: `BanUserFromCommentResult`

## Primer

[inline-code-attrs-start title = 'postBanUserFromComment Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
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