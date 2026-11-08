## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| banEmail | boolean | query | नहीं |  |
| banEmailDomain | boolean | query | नहीं |  |
| banIP | boolean | query | नहीं |  |
| deleteAllUsersComments | boolean | query | नहीं |  |
| bannedUntil | string | query | नहीं |  |
| isShadowBan | boolean | query | नहीं |  |
| updateId | string | query | नहीं |  |
| banReason | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `BanUserFromCommentResult`

## उदाहरण

[inline-code-attrs-start title = 'postBanUserFromComment उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
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