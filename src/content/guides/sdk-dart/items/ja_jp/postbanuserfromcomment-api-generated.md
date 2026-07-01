## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| banEmail | boolean | query | いいえ |  |
| banEmailDomain | boolean | query | いいえ |  |
| banIP | boolean | query | いいえ |  |
| deleteAllUsersComments | boolean | query | いいえ |  |
| bannedUntil | string | query | いいえ |  |
| isShadowBan | boolean | query | いいえ |  |
| updateId | string | query | いいえ |  |
| banReason | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: `BanUserFromCommentResult`

## 例

[inline-code-attrs-start title = 'postBanUserFromComment の例'; type = ''; isFunctional = false; inline-code-attrs-end]
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