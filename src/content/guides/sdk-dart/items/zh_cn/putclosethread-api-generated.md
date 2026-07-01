## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |

## 响应

返回: `APIEmptyResponse`

## 示例

[inline-code-attrs-start title = 'putCloseThread 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putCloseThread(tenantId, urlId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putCloseThread: $e\n');
}
[inline-code-end]