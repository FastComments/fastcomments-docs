当前页面的实时在线观众：指其 websocket 会话当前已订阅该页面的用户。  
返回 anonCount + totalCount（整个房间的订阅者，包括我们不枚举的匿名观众）。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 页面 URL 标识符（服务器端清理后）。 |
| afterName | string | query | No | 游标：传递上一次响应中的 nextAfterName。 |
| afterUserId | string | query | No | 游标的平局解决器：传递上一次响应中的 nextAfterUserId。当 afterName 被设置时必须提供，以防止名称平局导致条目丢失。 |

## 响应

Returns: `PageUsersOnlineResponse`

## 示例

[inline-code-attrs-start title = 'getOnlineUsers 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final urlId = urlId_example; // String | 页面 URL 标识符（服务器端清理后）。
final afterName = afterName_example; // String | 游标：传递上一次响应中的 nextAfterName。
final afterUserId = afterUserId_example; // String | 游标的平局解决器：传递上一次响应中的 nextAfterUserId。当 afterName 被设置时必须提供，以防止名称平局导致条目丢失。

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]