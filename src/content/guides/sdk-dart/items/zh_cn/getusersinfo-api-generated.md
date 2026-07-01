批量获取租户的用户信息。根据 userIds，返回来自 User / SSOUser 的显示信息。  
由评论小部件使用，以在用户通过 Presence 事件刚出现时丰富其信息。  
无页面上下文：隐私统一强制执行（私人资料会被遮蔽）。

## Parameters

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| ids | string | query | 是 | 逗号分隔的 userIds。 |

## Response

返回： `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'getUsersInfo 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | 逗号分隔的 userIds。

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]