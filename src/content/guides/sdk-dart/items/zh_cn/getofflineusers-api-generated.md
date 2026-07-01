Past commenters on the page who are NOT currently online. Sorted by displayName.  
页面上过去的评论者（当前未在线）。按 displayName 排序。  

Use this after exhausting /users/online to render a "Members" section.  
在使用完 /users/online 之后，使用它来渲染 “Members” 部分。  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
在 commenterName 上使用游标分页：服务器从 afterName 向前遍历部分 {tenantId, urlId, commenterName} 索引，使用 $gt，且没有 $skip 成本。  

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 页面 URL 标识符（服务器端已清理）。 |
| afterName | string | query | No | 游标：传递上一次响应中的 nextAfterName。 |
| afterUserId | string | query | No | 游标平局破局器：传递上一次响应中的 nextAfterUserId。当设置 afterName 时必须提供，以防止名称平局导致条目被丢弃。 |

## 响应

返回：`PageUsersOfflineResponse`

## 示例

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 页面 URL 标识符（服务器端已清理）。
final afterName = afterName_example; // String | 游标：传递上一次响应中的 nextAfterName。
final afterUserId = afterUserId_example; // String | 游标平局破局器：传递上一次响应中的 nextAfterUserId。当设置 afterName 时必须提供，以防止名称平局导致条目被丢弃。

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]