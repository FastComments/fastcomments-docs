## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| notificationId | string | path | 是 |  |
| newStatus | string | path | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回: `UpdateUserNotificationStatusResponse`

## 示例

[inline-code-attrs-start title = 'updateUserNotificationStatus 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // 字符串 |
final notificationId = notificationId_example; // 字符串 |
final newStatus = newStatus_example; // 字符串 |
final sso = sso_example; // 字符串 |

try {
    final result = api_instance.updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->updateUserNotificationStatus: $e\n');
}
[inline-code-end]