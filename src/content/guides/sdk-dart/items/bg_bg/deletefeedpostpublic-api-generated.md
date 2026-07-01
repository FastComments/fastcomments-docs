## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| postId | string | path | Да |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: `DeleteFeedPostPublicResponse`

## Пример

[inline-code-attrs-start title = 'deleteFeedPostPublic Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postId = postId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteFeedPostPublic(tenantId, postId, DeleteFeedPostPublicOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteFeedPostPublic: $e\n');
}
[inline-code-end]