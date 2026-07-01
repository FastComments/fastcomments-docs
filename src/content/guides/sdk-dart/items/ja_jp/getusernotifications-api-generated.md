## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| urlId | string | query | いいえ | 現在のページが購読されているかどうかを判定するために使用されます。 |
| pageSize | integer | query | いいえ |  |
| afterId | string | query | いいえ |  |
| includeContext | boolean | query | いいえ |  |
| afterCreatedAt | integer | query | いいえ |  |
| unreadOnly | boolean | query | いいえ |  |
| dmOnly | boolean | query | いいえ |  |
| noDm | boolean | query | いいえ |  |
| includeTranslations | boolean | query | いいえ |  |
| includeTenantNotifications | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## 応答

返却: `GetMyNotificationsResponse`

## 例

[inline-code-attrs-start title = 'getUserNotifications の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 現在のページが購読されているかどうかを判定するために使用されます。
final pageSize = 56; // int | 
final afterId = afterId_example; // String | 
final includeContext = true; // bool | 
final afterCreatedAt = 789; // int | 
final unreadOnly = true; // bool | 
final dmOnly = true; // bool | 
final noDm = true; // bool | 
final includeTranslations = true; // bool | 
final includeTenantNotifications = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserNotifications(tenantId, GetUserNotificationsOptions(urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserNotifications: $e\n');
}
[inline-code-end]