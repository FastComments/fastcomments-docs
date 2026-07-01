## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回: `GetBannedUsersCountResponse`

## 示例

[inline-code-attrs-start title = 'getCounts 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // 字符串 | 
final sso = sso_example; // 字符串 | 

try {
    final result = api_instance.getCounts(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getCounts: $e\n');
}
[inline-code-end]