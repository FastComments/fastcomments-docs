## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| trustFactor | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回： `SetUserTrustFactorResponse`

## 示例

[inline-code-attrs-start title = 'setTrustFactor 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final trustFactor = trustFactor_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.setTrustFactor(tenantId, SetTrustFactorOptions(userId: userId, trustFactor: trustFactor, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->setTrustFactor: $e\n');
}
[inline-code-end]

---