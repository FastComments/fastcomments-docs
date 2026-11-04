页面上当前不在线的历史评论者。按 displayName 排序。
在用尽 /users/online 之后，使用此接口渲染“成员”部分。
基于 commenterName 的游标分页：服务器通过 $gt 从 afterName 开始沿部分索引 {tenantId, urlId, commenterName} 向前遍历，没有 $skip 开销。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|------|------|------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（在服务端进行清理）。 |
| afterName | string | query | 否 | 游标：传入上一次响应中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游标决胜键：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时必填，以避免同名条目被丢弃。 |

## 响应

返回：`PageUsersOfflineResponse`

## 示例

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 页面 URL 标识符（在服务端进行清理）。
final afterName = afterName_example; // String | 游标：传入上一次响应中的 nextAfterName。
final afterUserId = afterUserId_example; // String | 游标决胜键：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时必填，以避免同名条目被丢弃。

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]