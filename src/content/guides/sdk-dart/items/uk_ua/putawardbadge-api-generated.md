## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| badgeId | string | query | Так |  |
| userId | string | query | Ні |  |
| commentId | string | query | Ні |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `AwardUserBadgeResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад putAwardBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final badgeId = badgeId_example; // String | 
final userId = userId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putAwardBadge(tenantId, badgeId, PutAwardBadgeOptions(userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putAwardBadge: $e\n');
}
[inline-code-end]