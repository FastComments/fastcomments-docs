## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| badgesUserId | string | query | Ні |  |
| commentId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `GetUserManualBadgesResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getManualBadgesForUser'; type = ''; isFunctional = false; inline-code-attrs-end]
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